use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Debug, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl const Default for Vec3 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl const std::ops::Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl const std::ops::Add<f32> for Vec3 {
    type Output = Quat;
    #[inline]
    fn add(self, rhs: f32) -> Quat {
        Quat::from(self) + rhs
    }
}

impl const std::ops::Add<Vec3> for f32 {
    type Output = Quat;
    #[inline]
    fn add(self, rhs: Vec3) -> Quat {
        rhs + self
    }
}

impl std::ops::AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::iter::Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), std::ops::Add::add)
    }
}

impl const std::ops::Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl const std::ops::Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl const std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        rhs * self
    }
}

impl const std::ops::Mul<Quat> for Vec3 {
    type Output = Quat;
    #[inline]
    fn mul(self, rhs: Quat) -> Quat {
        Quat::from(self) * rhs
    }
}

impl const std::ops::Div<f32> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl const From<Quat> for Vec3 {
    #[inline]
    fn from(quat: Quat) -> Self {
        Self::new(quat.i, quat.j, quat.k)
    }
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const I: Self = Self::new(1.0, 0.0, 0.0);
    pub const J: Self = Self::new(0.0, 1.0, 0.0);
    pub const K: Self = Self::new(0.0, 0.0, 1.0);

    #[inline]
    pub const fn sq_mag(self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn mag(self) -> f32 {
        self.sq_mag().sqrt()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        self / self.mag()
    }

    #[inline]
    pub fn rotate(self, rot: Quat) -> Self {
        if rot == Quat::ONE {
            self
        } else {
            Self::from(rot * self * rot.conj())
        }
    }

    #[inline]
    pub const fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub const fn cross(self, rhs: Self) -> Self {
        Self::from(self * Quat::from(rhs))
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
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl const std::ops::Add for Quat {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.r + rhs.r,
            self.i + rhs.i,
            self.j + rhs.j,
            self.k + rhs.k,
        )
    }
}

impl const std::ops::Add<f32> for Quat {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f32) -> Self {
        self + Self::from(rhs)
    }
}

impl const std::ops::Add<Quat> for f32 {
    type Output = Quat;
    #[inline]
    fn add(self, rhs: Quat) -> Quat {
        rhs + self
    }
}

impl const std::ops::Sub for Quat {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.r - rhs.r,
            self.i - rhs.i,
            self.j - rhs.j,
            self.k - rhs.k,
        )
    }
}

impl const std::ops::Neg for Quat {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(-self.r, -self.i, -self.j, -self.k)
    }
}

impl const std::ops::Mul for Quat {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.r * rhs.r - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            self.r * rhs.i + self.i * rhs.r + self.j * rhs.k - self.k * rhs.j,
            self.r * rhs.j - self.i * rhs.k + self.j * rhs.r + self.k * rhs.i,
            self.r * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.r,
        )
    }
}

impl const std::ops::Mul<f32> for Quat {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.r * rhs, self.i * rhs, self.j * rhs, self.k * rhs)
    }
}

impl const std::ops::Mul<Vec3> for Quat {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self {
        self * Self::from(rhs)
    }
}

impl std::ops::MulAssign<f32> for Quat {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl const std::ops::Mul<Quat> for f32 {
    type Output = Quat;
    #[inline]
    fn mul(self, rhs: Quat) -> Quat {
        rhs * self
    }
}

impl const From<Vec3> for Quat {
    #[inline]
    fn from(vec: Vec3) -> Self {
        Self::new(0.0, vec.x, vec.y, vec.z)
    }
}

impl const From<f32> for Quat {
    #[inline]
    fn from(r: f32) -> Self {
        Self::new(r, 0.0, 0.0, 0.0)
    }
}

impl Quat {
    pub const fn new(r: f32, i: f32, j: f32, k: f32) -> Self {
        Self { r, i, j, k }
    }

    pub const ONE: Self = Self::new(1.0, 0.0, 0.0, 0.0);

    pub fn rotation(axis: Vec3, angle: f32) -> Self {
        let hf_angle = angle / 2.0;
        hf_angle.cos() + axis * hf_angle.sin()
    }

    pub const fn conj(self) -> Self {
        Self::new(self.r, -self.i, -self.j, -self.k)
    }

    #[inline]
    pub const fn sq_mag(self) -> f32 {
        self.r * self.r + self.i * self.i + self.j * self.j + self.k * self.k
    }

    #[inline]
    pub fn mag(self) -> f32 {
        self.sq_mag().sqrt()
    }
}

#[cfg(test)]
mod vec3_tests {
    use super::*;

    #[test]
    fn new() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert!((a.x - 1.0).abs() < f32::EPSILON);
        assert!((a.y - 2.0).abs() < f32::EPSILON);
        assert!((a.z - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn add_vec3() {
        let a: Vec3 = Vec3::new(0.5, 0.5, 1.0);
        let b: Vec3 = Vec3::new(1.5, 1.0, 2.0);
        let c: Vec3 = a + b;
        assert_eq!(c, Vec3::new(2.0, 1.5, 3.0));
    }

    #[test]
    fn add_f32() {
        let a: Vec3 = Vec3::new(1.0, 1.0, 2.0);
        let b: f32 = 2.0;
        let c = a + b;
        assert_eq!(c, Quat::new(2.0, 1.0, 1.0, 2.0));
    }

    #[test]
    fn subtract() {
        let a: Vec3 = Vec3::new(1.0, 3.0, 1.0);
        let b: Vec3 = Vec3::new(0.5, 1.2, 1.5);
        let c: Vec3 = a - b;
        assert_eq!(c, Vec3::new(0.5, 1.8, -0.5));
    }

    #[test]
    fn neg() {
        let a: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let b = -a;
        assert_eq!(b, Vec3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn mul_f32() {
        let a: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let b: f32 = 10.0;
        let c: Vec3 = a * b;
        assert_eq!(c, Vec3::new(10.0, 20.0, 30.0));
    }

    #[test]
    fn mul_quat() {
        let a: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let b: Quat = Quat::new(1.0, 0.5, 2.0, 1.5);
        let c: Quat = a * b;
        assert_eq!(c, Quat::new(-9.0, -2.0, 2.0, 4.0));
    }

    #[test]
    fn sq_mag() {
        let a: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let b: f32 = a.sq_mag();
        assert!((b - 14.0).abs() < f32::EPSILON);
    }

    #[test]
    fn mag() {
        let a: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let b: f32 = a.mag();
        assert!((b - 14.0f32.sqrt()).abs() < f32::EPSILON);
    }

    #[test]
    fn rotate() {
        let v: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        let rot: Quat = Quat::rotation(Vec3::K, std::f32::consts::FRAC_PI_2);
        let v_new: Vec3 = v.rotate(rot);
        assert!((v_new - Vec3::new(-1.0, 1.0, 1.0)).sq_mag() < f32::EPSILON);
    }
}

#[cfg(test)]
mod quat_tests {
    use super::*;

    #[test]
    fn new() {
        let a: Quat = Quat::new(1.0, 2.0, 3.0, 4.0);
        assert!((a.r - 1.0).abs() < f32::EPSILON);
        assert!((a.i - 2.0).abs() < f32::EPSILON);
        assert!((a.j - 3.0).abs() < f32::EPSILON);
        assert!((a.k - 4.0).abs() < f32::EPSILON);
    }

    #[test]
    fn add_quat() {
        let a: Quat = Quat::new(1.0, 1.0, 1.0, 1.0);
        let b: Quat = Quat::new(0.5, 1.0, 0.5, 1.0);
        let c: Quat = a + b;
        assert!((c - Quat::new(1.5, 2.0, 1.5, 2.0)).sq_mag() < f32::EPSILON);
    }

    #[test]
    fn add_f32() {
        let a: Quat = Quat::new(1.0, 0.5, 1.0, 2.0);
        let b: f32 = 10.0;
        let c: Quat = a + b;
        assert!((c - Quat::new(11.0, 0.5, 1.0, 2.0)).sq_mag() < f32::EPSILON);
    }

    #[test]
    fn sub() {
        let a: Quat = Quat::new(1.0, 1.0, 1.0, 1.0);
        let b: Quat = Quat::new(0.5, 1.0, 0.5, 1.0);
        let c: Quat = a - b;
        assert!((c - Quat::new(0.5, 0.0, 0.5, 0.0)).sq_mag() < f32::EPSILON);
    }

    #[test]
    fn neg() {
        let a: Quat = Quat::new(1.0, 1.0, 1.0, 1.0);
        let b: Quat = -a;
        assert!((b - Quat::new(-1.0, -1.0, -1.0, -1.0)).sq_mag() < f32::EPSILON);
    }

    #[test]
    fn mul_quat() {
        let a: Quat = Quat::new(1.0, 1.0, 1.0, 1.0);
        let b: Quat = Quat::new(0.5, 1.0, 0.5, 1.0);
        let c: Quat = a * b;
        assert!((c - Quat::new(-2.0, 2.0, 1.0, 1.0)).sq_mag() < f32::EPSILON);
    }

    #[test]
    fn mul_f32() {
        let a: Quat = Quat::new(0.5, 1.0, 0.5, 1.0);
        let b: f32 = 2.0;
        let c: Quat = a * b;
        assert!((c - Quat::new(1.0, 2.0, 1.0, 2.0)).sq_mag() < f32::EPSILON);
    }

    #[test]
    fn sq_mag() {
        let a: Quat = Quat::new(1.0, 2.0, 3.0, 4.0);
        let b: f32 = a.sq_mag();
        assert!((b - 30.0).abs() < f32::EPSILON);
    }

    #[test]
    fn mag() {
        let a: Quat = Quat::new(1.0, 2.0, 3.0, 4.0);
        let b: f32 = a.mag();
        assert!((b - 30.0f32.sqrt()).abs() < f32::EPSILON);
    }
}
