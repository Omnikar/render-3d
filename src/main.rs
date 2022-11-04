use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{self, Color, Stylize},
    terminal, QueueableCommand, Result,
};
use std::io::{stdout, Write};

const DIMS: (u16, u16) = (50, 50);

fn main() -> Result<()> {
    let world = World::default();
    let mut camera = Camera {
        transform: Transform {
            position: -0.8 * Vec3::i(),
            rotation: Quat::one(),
        },
        px_per_unit: 20.0,
        focal_length: 2.0,
    };

    crossterm::terminal::enable_raw_mode()?;

    let mut stdout = std::io::BufWriter::new(stdout());

    execute!(stdout, cursor::Hide, terminal::EnterAlternateScreen)?;

    queue_render(&mut stdout, &world, &camera)?;
    stdout.flush()?;

    while let Ok(event) = event::read() {
        if let Event::Key(event) = event {
            if event.code == KeyCode::Char('c') {
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
                    camera.transform.position.x += 0.1;
                    camera.focal_length -= 0.1;
                }
                KeyCode::Char('z') => {
                    camera.transform.position.x -= 0.1;
                    camera.focal_length += 0.1;
                }
                _ => (),
            }

            let mut rotation = |angle: f32, axis: Vec3| {
                let hf_angle = angle / 2.0;
                let new_rot = Quat::from(hf_angle.cos()) + Quat::from(axis) * hf_angle.sin();

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

fn queue_render(mut stdout: impl Write, world: &World, camera: &Camera) -> Result<()> {
    for (y_hf, x) in (0..DIMS.1 / 2).flat_map(|y_hf| std::iter::repeat(y_hf).zip(0..DIMS.0)) {
        let x_fl = x as f32 - (DIMS.0 as f32) / 2.0;
        let y_hf_fl = y_hf as f32 - (DIMS.1 as f32) / 4.0;
        stdout
            .queue(cursor::MoveTo(x, y_hf))?
            .queue(style::PrintStyledContent(
                "▀"
                    .with(camera.get_px(world, x_fl, y_hf_fl * 2.0))
                    .on(camera.get_px(world, x_fl, y_hf_fl * 2.0 + 1.0)),
            ))?;
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Default for Vec3 {
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
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
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
    fn neg(self) -> Vec3 {
        Vec3::default() - self
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
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
    fn mul(self, rhs: f32) -> Vec3 {
        rhs * self
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Vec3 {
    fn i() -> Vec3 {
        Vec3 {
            x: 1.0,
            ..Vec3::default()
        }
    }

    fn j() -> Vec3 {
        Vec3 {
            y: 1.0,
            ..Vec3::default()
        }
    }

    fn k() -> Vec3 {
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
        Vec3::from(rot * Quat::from(self) * rot.conj())
    }
}

impl From<Quat> for Vec3 {
    fn from(quat: Quat) -> Vec3 {
        Vec3 {
            x: quat.i,
            y: quat.j,
            z: quat.k,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Quat {
    r: f32,
    i: f32,
    j: f32,
    k: f32,
}

impl Default for Quat {
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
    fn add(self, rhs: Quat) -> Quat {
        Quat {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

impl std::ops::Sub for Quat {
    type Output = Quat;
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
    fn neg(self) -> Quat {
        Quat::default() - self
    }
}

impl std::ops::Mul for Quat {
    type Output = Quat;
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
    fn mul(self, rhs: f32) -> Quat {
        Quat {
            r: self.r * rhs,
            i: self.i * rhs,
            j: self.j * rhs,
            k: self.k * rhs,
        }
    }
}

impl std::ops::MulAssign<f32> for Quat {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl std::ops::Mul<Quat> for f32 {
    type Output = Quat;
    fn mul(self, rhs: Quat) -> Quat {
        rhs * self
    }
}

impl From<Vec3> for Quat {
    fn from(vec: Vec3) -> Quat {
        Quat {
            r: 0.0,
            i: vec.x,
            j: vec.y,
            k: vec.z,
        }
    }
}

impl From<f32> for Quat {
    fn from(r: f32) -> Quat {
        Quat {
            r,
            ..Quat::default()
        }
    }
}

impl Quat {
    fn one() -> Quat {
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
    fn get_px(&self, world: &World, x: f32, y: f32) -> Color {
        let step_size = 0.1f32;
        let max_length = 10.0f32;
        let max_steps = (max_length / step_size).ceil() as usize;

        let step = (Vec3 {
            x: self.focal_length,
            y: -x / self.px_per_unit,
            z: -y / self.px_per_unit,
        }
        .normalize()
            * step_size)
            .rotate(self.transform.rotation);

        let mut current = self.transform.position;
        for _ in 0..max_steps {
            current += step;
            if let Some((.., color)) = world.cuboids.iter().find(|(c1, c2, _)| {
                ((current.x - c1.x).is_sign_positive() ^ (current.x - c2.x).is_sign_positive())
                    && ((current.y - c1.y).is_sign_positive()
                        ^ (current.y - c2.y).is_sign_positive())
                    && ((current.z - c1.z).is_sign_positive()
                        ^ (current.z - c2.z).is_sign_positive())
            }) {
                return *color;
            } else if let Some((.., color)) = world.spheres.iter().find(|(pos, r, _)| {
                ((pos.x - current.x).abs() <= *r
                    && (pos.y - current.y).abs() <= *r
                    && (pos.z - current.z).abs() <= *r)
                    .then(|| (current - *pos).sq_mag() <= r.powi(2))
                    .unwrap_or_default()
            }) {
                return *color;
            }
        }

        Color::Black
    }
}

struct World {
    spheres: Vec<(Vec3, f32, Color)>,
    cuboids: Vec<(Vec3, Vec3, Color)>,
}

impl Default for World {
    fn default() -> Self {
        let spheres = vec![
            (
                Vec3 {
                    x: 7.0,
                    y: 0.0,
                    z: 4.0,
                },
                1.0,
                Color::Red,
            ),
            (
                Vec3 {
                    x: 7.0,
                    y: 0.0,
                    z: -4.0,
                },
                1.0,
                Color::Red,
            ),
            (
                Vec3 {
                    x: 1.0,
                    y: -0.5,
                    z: 0.0,
                },
                0.2,
                Color::Blue,
            ),
            (
                Vec3 {
                    x: 1.0,
                    y: 0.5,
                    z: 0.0,
                },
                0.2,
                Color::Yellow,
            ),
            (
                Vec3 {
                    x: 2.0,
                    y: -0.5,
                    z: 0.0,
                },
                0.2,
                Color::Yellow,
            ),
            (
                Vec3 {
                    x: 2.0,
                    y: 0.5,
                    z: 0.0,
                },
                0.2,
                Color::Blue,
            ),
            (
                Vec3 {
                    x: 3.0,
                    y: -0.5,
                    z: 0.0,
                },
                0.2,
                Color::Blue,
            ),
            (
                Vec3 {
                    x: 3.0,
                    y: 0.5,
                    z: 0.0,
                },
                0.2,
                Color::Yellow,
            ),
            (
                Vec3 {
                    x: 4.5,
                    y: -0.7,
                    z: 0.0,
                },
                0.4,
                Color::Green,
            ),
            (
                Vec3 {
                    x: 4.5,
                    y: 0.7,
                    z: 0.0,
                },
                0.4,
                Color::Green,
            ),
        ];
        let cuboids = vec![(
            Vec3 {
                x: 1.0,
                y: -1.0,
                z: -0.1,
            },
            Vec3 {
                x: 1.2,
                y: -1.2,
                z: 0.1,
            },
            Color::Magenta,
        )];
        Self { spheres, cuboids }
    }
}
