use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Debug, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
    pub const fn i() -> Vec3 {
        Vec3 {
            x: 1.0,
            ..Vec3::default()
        }
    }

    pub const fn j() -> Vec3 {
        Vec3 {
            y: 1.0,
            ..Vec3::default()
        }
    }

    pub const fn k() -> Vec3 {
        Vec3 {
            z: 1.0,
            ..Vec3::default()
        }
    }

    #[inline]
    pub fn sq_mag(self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    #[inline]
    pub fn mag(self) -> f32 {
        self.sq_mag().sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Vec3 {
        self / self.mag()
    }

    #[inline]
    pub fn rotate(self, rot: Quat) -> Vec3 {
        if rot == Quat::one() {
            self
        } else {
            Vec3::from(rot * self * rot.conj())
        }
    }

    #[inline]
    pub fn dot(self, rhs: Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3::from(self * Quat::from(rhs))
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quat {
    pub r: f32,
    pub i: f32,
    pub j: f32,
    pub k: f32,
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
    /// let a: Quat = Quat { r: 0.0, i: 1.0, j: 0.0, k: 1.0};
    /// let b: Quat = Quat { r: 1.0, i: 0.0, j: 1.0, k: 0.0};
    /// let c: Quat = a + b;
    /// let expected = Quat { r: 1.0, i: 1.0, j: 1.0, k: 1.0};
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

impl const From<Vec3> for Quat {
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
    pub const fn one() -> Quat {
        Quat {
            r: 1.0,
            ..Quat::default()
        }
    }

    pub fn conj(self) -> Quat {
        Quat {
            r: self.r,
            i: -self.i,
            j: -self.j,
            k: -self.k,
        }
    }

    #[inline]
    pub fn sq_mag(self) -> f32 {
        self.r.powi(2) + self.i.powi(2) + self.j.powi(2) + self.k.powi(2)
    }

    #[inline]
    pub fn mag(self) -> f32 {
        self.sq_mag().sqrt()
    }
}
