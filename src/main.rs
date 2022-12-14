// Requires nightly, allows for constant implementations of traits, for Self::Default for storage and small perf improvements.
#![feature(
    const_trait_impl,
    const_fn_floating_point_arithmetic,
    core_intrinsics,
    is_some_and
)]
// `clippy::pedantic` with exceptions
#![warn(clippy::pedantic)]
#![allow(
    clippy::items_after_statements,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_lossless
)]

mod camera;
mod math;
mod world;

use camera::Camera;
use math::{Quat, Vec3};
use world::{Transform, World};

use pixels::{PixelsBuilder, SurfaceTexture};
use rayon::prelude::*;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

/// Dimentions of the Window (in pixels), width by height
const DIMS: (u32, u32) = (600, 375);
const HALF_DIMS: (f32, f32) = (DIMS.0 as f32 / 2.0, DIMS.1 as f32 / 2.0);

/// Number of frames used to create average
const N_FRAMES: usize = 20;

fn main() {
    let world = ron::from_str::<World>(include_str!("../scenes/cube.ron"))
        .expect("failed to parse World file");
    let mut camera = Camera {
        transform: Transform {
            position: -0.8 * Vec3::J,
            rotation: Quat::ONE,
        },
        px_per_unit: 160.0,
        focal_length: 2.0,
    };

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(DIMS.0, DIMS.1);
        WindowBuilder::new()
            .with_title("Raytracing Test")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_decorations(false) // weird graphical issue happens without this (at least on gnome + wayland) further investigation needed
            .build(&event_loop)
            .expect("WindowBuilder failed")
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        PixelsBuilder::new(DIMS.0, DIMS.1, surface_texture)
            .enable_vsync(true)
            .build()
            .expect("failed to create pixels")
    };
    // Fill alpha channel to avoid setting it later
    pixels.get_frame_mut().fill(0xff);

    let mut frametime_log: VecDeque<Duration> = VecDeque::with_capacity(N_FRAMES);

    event_loop.run(move |event, _, control_flow| {
        let keyboard_input: bool =
            input.update(&event) && handle_input(&input, control_flow, &mut camera);

        let redraw_requested: bool = matches!(event, Event::RedrawRequested(_));

        // Draw the current frame
        if keyboard_input || redraw_requested {
            do_render(
                pixels.get_frame_mut(),
                &world,
                &camera,
                Some(&mut frametime_log),
            );
            if pixels
                .render()
                .map_err(|e| panic!("pixels.render() failed: {e}"))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
            }
        }
    });
}

fn handle_input(
    input: &WinitInputHelper,
    control_flow: &mut ControlFlow,
    camera: &mut Camera,
) -> bool {
    if (input.key_held(VirtualKeyCode::LControl) || input.key_held(VirtualKeyCode::RControl))
        && input.key_pressed(VirtualKeyCode::C)
    {
        *control_flow = ControlFlow::Exit;
    }
    const DELTA: f32 = 0.015;
    const MOVE_SPEED: f32 = 3.0 * DELTA;
    const TURN_SPEED: f32 = std::f32::consts::FRAC_PI_2 * DELTA;
    let mut did_movement: bool = false;
    let mut movement = |delta: Vec3| {
        camera.transform.position += delta.rotate(camera.transform.rotation);
        did_movement = true;
    };
    [
        (VirtualKeyCode::W, Vec3::J),
        (VirtualKeyCode::S, -Vec3::J),
        (VirtualKeyCode::D, Vec3::I),
        (VirtualKeyCode::A, -Vec3::I),
        (VirtualKeyCode::E, Vec3::K),
        (VirtualKeyCode::Q, -Vec3::K),
    ]
    .into_iter()
    .filter_map(|(key, axis)| input.key_held(key).then_some(axis))
    .for_each(|axis| movement(MOVE_SPEED * axis));
    if input.key_held(VirtualKeyCode::X) {
        movement(MOVE_SPEED * Vec3::J);
        camera.focal_length -= MOVE_SPEED;
    }
    if input.key_held(VirtualKeyCode::Z) {
        movement(-MOVE_SPEED * Vec3::J);
        camera.focal_length += MOVE_SPEED;
    }
    if input.key_held(VirtualKeyCode::R) {
        camera.focal_length += MOVE_SPEED;
        did_movement = true;
    }
    if input.key_held(VirtualKeyCode::F) {
        camera.focal_length -= MOVE_SPEED;
        did_movement = true;
    }
    let mut did_rotation: bool = false;
    let mut rotation = |angle: f32, axis: Vec3| {
        let rot = &mut camera.transform.rotation;
        let mut new_rot = Quat::rotation(axis.rotate(*rot), angle);

        // Mathematically, the magnitude should always remain at 1 already, but floating point
        // precision errors may cause self-fueleing inaccuracy that becomes worse with each rotation.
        if (new_rot.sq_mag() - 1.0).abs() > f32::EPSILON {
            new_rot = new_rot * new_rot.mag().recip();
        }

        *rot = new_rot * *rot;
        did_rotation = true;
    };
    [
        (VirtualKeyCode::J, Vec3::K),
        (VirtualKeyCode::L, -Vec3::K),
        (VirtualKeyCode::I, Vec3::I),
        (VirtualKeyCode::K, -Vec3::I),
        (VirtualKeyCode::O, Vec3::J),
        (VirtualKeyCode::U, -Vec3::J),
    ]
    .into_iter()
    .filter_map(|(key, axis)| input.key_held(key).then_some(axis))
    .for_each(|axis| rotation(TURN_SPEED, axis));
    did_rotation || did_movement
}

fn do_render(
    frame: &mut [u8],
    world: &World,
    camera: &Camera,
    frame_data: Option<&mut VecDeque<Duration>>,
) {
    // Create a instant here to time how long it takes to render a frame
    let now = Instant::now();

    // Used to zip with frame data in place of enumerating (which cannot be done with par_chunks_exact_mut)
    const INDEX: std::ops::Range<u32> = 0..(DIMS.0 * DIMS.1);

    frame
        .par_chunks_exact_mut(4)
        .zip(INDEX)
        .for_each(|(pixel, i)| {
            // SAFETY: Pixel size will always be 4, RGBA
            unsafe {
                std::intrinsics::assume(pixel.len() == 4);
            }

            // (x, y) of pixel on screen
            let (x, y): (u32, u32) = (i % DIMS.0, i / DIMS.0);

            let x_w = x as f32 - HALF_DIMS.0;
            let y_w = y as f32 - HALF_DIMS.1;
            pixel[0..=2].copy_from_slice(&camera.get_px(world, x_w, y_w).0);
        });

    let took = now.elapsed();

    if let Some(frametime_log) = frame_data {
        // Only remove the last element if the queue is the desired size
        while frametime_log.len() >= N_FRAMES {
            frametime_log.pop_back();
        }
        frametime_log.push_front(took);

        // The length of `frametime_log` can never be longer than `N_FRAMES`
        #[allow(clippy::cast_possible_truncation)]
        let avg_frametime = frametime_log.iter().sum::<Duration>() / frametime_log.len() as u32;

        eprintln!("Frame took: {took:#?} (avg: {avg_frametime:#?})");
    } else {
        eprintln!("Frame took: {took:#?}");
    }
}
