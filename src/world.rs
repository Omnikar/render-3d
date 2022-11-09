use crate::math::{Quat, Vec3};

use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct World {
    pub spheres: Vec<(Vec3, f32, Color)>,
    pub tris: Vec<(Vec3, Vec3, Vec3, Color)>,
    pub light: Vec3,
}

pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
}

#[derive(Clone, Copy, Deserialize)]
pub struct Color(pub [u8; 3]);

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

impl std::ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Color {
        Color(self.0.map(|n| (n as f32 * rhs).round() as u8))
    }
}

impl Color {
    #[allow(dead_code)]
    fn interpolate(self, rhs: Color, ratio: f32) -> Color {
        Color([0, 1, 2].map(|i| {
            (self[i] as f32 * (1.0 - ratio)).round() as u8 + (rhs[i] as f32 * ratio).round() as u8
        }))
    }
}
