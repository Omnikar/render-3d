// Requires nightly, allows for constant implementations of traits, for Self::Default for storage and small perf improvements.
#![feature(const_trait_impl)]

// TODO: replace with `anyhow` crate
#[macro_use]
extern crate lazy_static;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;
use std::io::{stdout, Write};

const DIMS: (u16, u16) = (500, 250);

fn main() -> Result<()> {
    let world = ron::from_str::<World>(include_str!("../scenes/sample.ron")).unwrap();
    let mut camera = Camera {
        transform: Transform {
            position: -0.8 * Vec3::i(),
            rotation: Quat::one(),
        },
        px_per_unit: 40.0,
        focal_length: 2.0,
    };

    crossterm::terminal::enable_raw_mode()?;

    let mut stdout = std::io::BufWriter::new(stdout());

    execute!(stdout, cursor::Hide, terminal::EnterAlternateScreen)?;

    queue_render(&mut stdout, &world, &camera)?;
    stdout.flush()?;

    while let Ok(event) = event::read() {
        if let Event::Key(event) = event {
            if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c') {
                break;
            }

            let mut movement = |delta: Vec3| {
                camera.transform.position += delta.rotate(camera.transform.rotation);
            };

            match event.code {
                KeyCode::Char('w') => movement(0.1 * Vec3::i()),
                KeyCode::Char('s') => movement(-0.1 * Vec3::i()),
                KeyCode::Char('a') => movement(0.1 * Vec3::j()),
                KeyCode::Char('d') => movement(-0.1 * Vec3::j()),
                KeyCode::Char('e') => movement(0.1 * Vec3::k()),
                KeyCode::Char('q') => movement(-0.1 * Vec3::k()),
                KeyCode::Char('r') => camera.focal_length += 0.1,
                KeyCode::Char('f') => camera.focal_length -= 0.1,
                KeyCode::Char('x') => {
                    movement(0.1 * Vec3::i());
                    camera.focal_length -= 0.1;
                }
                KeyCode::Char('z') => {
                    movement(-0.1 * Vec3::i());
                    camera.focal_length += 0.1;
                }
                _ => (),
            }

            let mut rotation = |angle: f32, axis: Vec3| {
                let hf_angle = angle / 2.0;
                let new_rot = hf_angle.cos() + axis * hf_angle.sin();

                let rot = &mut camera.transform.rotation;
                let new_rot = *rot * new_rot * rot.conj();
                // Mathematically, the magnitude should always remain at 1 already, but floating point
                // precision errors cause self-fueleing inaccuracy that becomes worse with each rotation.
                let new_rot = new_rot * new_rot.mag().recip();
                *rot = new_rot * *rot;
            };

            match event.code {
                KeyCode::Char('j') => rotation(std::f32::consts::FRAC_PI_8 / 4.0, Vec3::k()),
                KeyCode::Char('l') => rotation(-std::f32::consts::FRAC_PI_8 / 4.0, Vec3::k()),
                KeyCode::Char('k') => rotation(std::f32::consts::FRAC_PI_8 / 4.0, Vec3::j()),
                KeyCode::Char('i') => rotation(-std::f32::consts::FRAC_PI_8 / 4.0, Vec3::j()),
                KeyCode::Char('o') => rotation(std::f32::consts::FRAC_PI_8 / 4.0, Vec3::i()),
                KeyCode::Char('u') => rotation(-std::f32::consts::FRAC_PI_8 / 4.0, Vec3::i()),
                _ => (),
            }

            queue_render(&mut stdout, &world, &camera)?;
            stdout.flush()?;
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;

    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

lazy_static! {
    pub static ref DATA: Vec<(u16, u16, f32, f32)> = {
        let a = (DIMS.0 as f32) / 2.0;
        let b = (DIMS.1 as f32) / 4.0;
        (0..DIMS.1 / 2)
            .flat_map(|y_hf| std::iter::repeat(y_hf).zip(0..DIMS.0))
            .map(|(y_hf, x)| {
                let x_fl = x as f32 - a;
                let y_hf_fl = y_hf as f32 - b;
                (y_hf, x, x_fl, y_hf_fl)
            })
            .collect::<Vec<(u16, u16, f32, f32)>>()
    };
    pub static ref DATA_LEN: usize = DATA.len();
}

fn queue_render(mut stdout: impl Write, world: &World, camera: &Camera) -> Result<()> {
    let colors: Vec<(Color, Color)> = DATA
        .clone()
        .into_par_iter()
        .map(|(_, _, x_fl, y_hf_fl)| camera.get_double_px(world, x_fl, y_hf_fl * 2.0))
        .collect::<Vec<(Color, Color)>>();

    for (i, (y_hf, x, _, _)) in DATA.iter().enumerate() {
        stdout
            .queue(cursor::MoveTo(*x, *y_hf))?
            .queue(style::PrintStyledContent(
                "▀".with(colors[i].0.into()).on(colors[i].1.into()),
            ))?;
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Debug, Deserialize)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl const Default for Vec3 {
    fn default() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Add<f32> for Vec3 {
    type Output = Quat;
    #[inline(always)]
    fn add(self, rhs: f32) -> Quat {
        Quat::from(self) + rhs
    }
}

impl std::ops::Add<Vec3> for f32 {
    type Output = Quat;
    #[inline(always)]
    fn add(self, rhs: Vec3) -> Quat {
        rhs + self
    }
}

impl std::ops::AddAssign for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Vec3 {
        rhs * self
    }
}

impl std::ops::Mul<Quat> for Vec3 {
    type Output = Quat;
    #[inline(always)]
    fn mul(self, rhs: Quat) -> Quat {
        Quat::from(self) * rhs
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl const From<Quat> for Vec3 {
    #[inline(always)]
    fn from(quat: Quat) -> Vec3 {
        Vec3 {
            x: quat.i,
            y: quat.j,
            z: quat.k,
        }
    }
}

impl Vec3 {
    const fn i() -> Vec3 {
        Vec3 {
            x: 1.0,
            ..Vec3::default()
        }
    }

    const fn j() -> Vec3 {
        Vec3 {
            y: 1.0,
            ..Vec3::default()
        }
    }

    const fn k() -> Vec3 {
        Vec3 {
            z: 1.0,
            ..Vec3::default()
        }
    }

    fn sq_mag(self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    fn mag(self) -> f32 {
        self.sq_mag().sqrt()
    }

    fn normalize(self) -> Vec3 {
        self / self.mag()
    }

    fn rotate(self, rot: Quat) -> Vec3 {
        Vec3::from(rot * self * rot.conj())
    }

    fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3::from(self * Quat::from(rhs))
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Quat {
    r: f32,
    i: f32,
    j: f32,
    k: f32,
}

impl const Default for Quat {
    fn default() -> Quat {
        Quat {
            r: 0.0,
            i: 0.0,
            j: 0.0,
            k: 0.0,
        }
    }
}

impl std::ops::Add for Quat {
    type Output = Quat;
    /// Adds two quats together
    /// ```
    /// let a: Quat = Quat {0.0, 1.0, 0.0, 1.0};
    /// let b: Quat = Quat {1.0, 0.0, 1.0, 0.0};
    /// let c: Quat = a + b;
    /// let expected = Quat {1.0, 1.0, 1.0, 1.0};
    /// assert_eq!(c, expected);
    /// ```
    #[inline(always)]
    fn add(self, rhs: Quat) -> Quat {
        Quat {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

impl std::ops::Add<f32> for Quat {
    type Output = Quat;
    #[inline(always)]
    fn add(self, rhs: f32) -> Quat {
        self + Quat::from(rhs)
    }
}

impl std::ops::Add<Quat> for f32 {
    type Output = Quat;
    #[inline(always)]
    fn add(self, rhs: Quat) -> Quat {
        rhs + self
    }
}

impl std::ops::Sub for Quat {
    type Output = Quat;
    #[inline(always)]
    fn sub(self, rhs: Quat) -> Quat {
        Quat {
            r: self.r - rhs.r,
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k,
        }
    }
}

impl std::ops::Neg for Quat {
    type Output = Quat;
    #[inline(always)]
    fn neg(self) -> Quat {
        Quat {
            r: -self.r,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }
}

impl std::ops::Mul for Quat {
    type Output = Quat;
    #[inline(always)]
    fn mul(self, rhs: Quat) -> Quat {
        Quat {
            r: self.r * rhs.r - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            i: self.r * rhs.i + self.i * rhs.r + self.j * rhs.k - self.k * rhs.j,
            j: self.r * rhs.j - self.i * rhs.k + self.j * rhs.r + self.k * rhs.i,
            k: self.r * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.r,
        }
    }
}

impl std::ops::Mul<f32> for Quat {
    type Output = Quat;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Quat {
        Quat {
            r: self.r * rhs,
            i: self.i * rhs,
            j: self.j * rhs,
            k: self.k * rhs,
        }
    }
}

impl std::ops::Mul<Vec3> for Quat {
    type Output = Quat;
    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Quat {
        self * Quat::from(rhs)
    }
}

impl std::ops::MulAssign<f32> for Quat {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl std::ops::Mul<Quat> for f32 {
    type Output = Quat;
    #[inline(always)]
    fn mul(self, rhs: Quat) -> Quat {
        rhs * self
    }
}

impl From<Vec3> for Quat {
    #[inline(always)]
    fn from(vec: Vec3) -> Quat {
        Quat {
            r: 0.0,
            i: vec.x,
            j: vec.y,
            k: vec.z,
        }
    }
}

impl const From<f32> for Quat {
    #[inline(always)]
    fn from(r: f32) -> Quat {
        Quat {
            r,
            ..Quat::default()
        }
    }
}

impl Quat {
    const fn one() -> Quat {
        Quat {
            r: 1.0,
            ..Quat::default()
        }
    }

    fn conj(self) -> Quat {
        Quat {
            r: self.r,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }

    fn sq_mag(self) -> f32 {
        self.r.powi(2) + self.i.powi(2) + self.j.powi(2) + self.k.powi(2)
    }

    fn mag(self) -> f32 {
        self.sq_mag().sqrt()
    }
}

#[derive(Clone, Copy, Deserialize)]
struct Color([u8; 3]);

impl std::ops::Index<usize> for Color {
    type Output = u8;
    fn index(&self, index: usize) -> &u8 {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.0[index]
    }
}

impl From<Color> for style::Color {
    fn from(color: Color) -> Self {
        style::Color::Rgb {
            r: color[0],
            g: color[1],
            b: color[2],
        }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        Color(self.0.map(|n| (n as f32 * rhs).round() as u8))
    }
}

impl Color {
    fn interpolate(self, rhs: Color, ratio: f32) -> Color {
        Color([0, 1, 2].map(|i| {
            (self[i] as f32 * (1.0 - ratio)).round() as u8 + (rhs[i] as f32 * ratio).round() as u8
        }))
    }
}

struct Transform {
    position: Vec3,
    rotation: Quat,
}

struct Camera {
    transform: Transform,
    px_per_unit: f32,
    focal_length: f32,
}

impl Camera {
    /// Helper for get_px
    fn get_double_px(&self, world: &World, x: f32, y: f32) -> (Color, Color) {
        return (self.get_px(world, x, y), self.get_px(world, x, y + 1.0));
    }

    fn get_px(&self, world: &World, x: f32, y: f32) -> Color {
        let ray = Vec3 {
            x: self.focal_length,
            y: -x / self.px_per_unit,
            z: -y / self.px_per_unit,
        }
        .rotate(self.transform.rotation);

        let tris = world
            .tris
            .iter()
            .filter_map(|p| self.tri_raycast(ray, world.light, *p));
        let spheres = world
            .spheres
            .iter()
            .filter_map(|p| self.sphere_raycast(ray, world.light, *p));
        tris.chain(spheres)
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(color, _)| color)
            .unwrap_or(Color([0; 3]))
    }

    fn tri_raycast(
        &self,
        ray: Vec3,
        light: Vec3,
        (p1, p2, p3, color): (Vec3, Vec3, Vec3, Color),
    ) -> Option<(Color, f32)> {
        let pos = self.transform.position;
        // Check if within plane
        let v1 = p2 - p1;
        let v2 = p3 - p1;
        let dist = pos - p1;
        let cross = v1.cross(v2);
        let t = -cross.dot(dist) / cross.dot(ray);

        if !t.is_finite() || !t.is_sign_positive() {
            return None;
        }

        // Check if within tetrahedron
        let [(a, d, g), (b, e, h), (c, f, i)] =
            [p1, p2, p3].map(|p| p - pos).map(|v| (v.x, v.y, v.z));

        let det_neg =
            (a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)).is_sign_negative();
        if ![
            ray.x * (e * i - f * h) + ray.y * (c * h - b * i) + ray.z * (b * f - c * e),
            ray.x * (f * g - d * i) + ray.y * (a * i - c * g) + ray.z * (c * d - a * f),
            ray.x * (d * h - e * g) + ray.y * (b * g - a * h) + ray.z * (a * e - b * d),
        ]
        .iter()
        .all(|n| n.is_sign_positive() ^ det_neg)
        {
            return None;
        }

        let coord = self.transform.position + ray * t;
        let normal = v1.cross(v2).normalize();
        let light_vec = (light - coord).normalize();
        let illumination = light_vec.dot(normal).max(0.0);
        let color = color * illumination;

        Some((color, t))
    }

    fn sphere_raycast(
        &self,
        ray: Vec3,
        light: Vec3,
        (center, r, color): (Vec3, f32, Color),
    ) -> Option<(Color, f32)> {
        let dist = center - self.transform.position;
        let a = ray.sq_mag();
        let b = ray.dot(dist);
        let c = dist.sq_mag() - r.powi(2);

        let sqrt_term = (b.powi(2) - a * c).sqrt();
        if !sqrt_term.is_finite() {
            return None;
        }

        let t = [(b + sqrt_term) / a, (b - sqrt_term) / a]
            .into_iter()
            .filter(|n| n.is_sign_positive())
            .min_by(f32::total_cmp)?;
        let coord = self.transform.position + ray * t;
        let normal = (coord - center).normalize();
        let light_vec = (light - coord).normalize();
        let illumination = light_vec.dot(normal).max(0.0);
        let color = color * illumination;
        Some((color, t))
    }
}

#[derive(Default, Deserialize)]
struct World {
    spheres: Vec<(Vec3, f32, Color)>,
    tris: Vec<(Vec3, Vec3, Vec3, Color)>,
    light: Vec3,
}
