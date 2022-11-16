use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Debug, Deserialize)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl const Default for Vec3 {
    fn default() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl const std::ops::Add for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl const std::ops::Add<f32> for Vec3 {
    type Output = Quat;
    #[inline(always)]
    fn add(self, rhs: f32) -> Quat {
        Quat::from(self) + rhs
    }
}

impl const std::ops::Add<Vec3> for f32 {
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

impl const std::ops::Sub for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl const std::ops::Neg for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl const std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl const std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Vec3 {
        rhs * self
    }
}

impl const std::ops::Mul<Quat> for Vec3 {
    type Output = Quat;
    #[inline(always)]
    fn mul(self, rhs: Quat) -> Quat {
        Quat::from(self) * rhs
    }
}

impl const std::ops::Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn div(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl const From<Quat> for Vec3 {
    #[inline(always)]
    fn from(quat: Quat) -> Vec3 {
        Vec3::new(quat.i, quat.j, quat.k)
    }
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub const I: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    pub const J: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    pub const K: Vec3 = Vec3::new(0.0, 0.0, 1.0);

    #[inline]
    pub const fn sq_mag(self) -> f32 {
        self.dot(self)
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
        if rot == Quat::ONE {
            self
        } else {
            Vec3::from(rot * self * rot.conj())
        }
    }

    #[inline]
    pub const fn dot(self, rhs: Vec3) -> f32 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    #[inline]
    pub const fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3::from(self * Quat::from(rhs))
    }

    #[inline]
    pub const fn x(self) -> f32 {
        self.x
    }

    #[inline]
    pub const fn y(self) -> f32 {
        self.y
    }

    #[inline]
    pub const fn z(self) -> f32 {
        self.z
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quat {
    r: f32,
    i: f32,
    j: f32,
    k: f32,
}

impl const Default for Quat {
    fn default() -> Quat {
        Quat::new(0.0, 0.0, 0.0, 0.0)
    }
}

impl const std::ops::Add for Quat {
    type Output = Quat;
    #[inline(always)]
    fn add(self, rhs: Quat) -> Quat {
        Quat::new(
            self.r() + rhs.r(),
            self.i() + rhs.i(),
            self.j() + rhs.j(),
            self.k() + rhs.k(),
        )
    }
}

impl const std::ops::Add<f32> for Quat {
    type Output = Quat;
    #[inline(always)]
    fn add(self, rhs: f32) -> Quat {
        self + Quat::from(rhs)
    }
}

impl const std::ops::Add<Quat> for f32 {
    type Output = Quat;
    #[inline(always)]
    fn add(self, rhs: Quat) -> Quat {
        rhs + self
    }
}

impl const std::ops::Sub for Quat {
    type Output = Quat;
    #[inline(always)]
    fn sub(self, rhs: Quat) -> Quat {
        Quat::new(
            self.r() - rhs.r(),
            self.i() - rhs.i(),
            self.j() - rhs.j(),
            self.k() - rhs.k(),
        )
    }
}

impl const std::ops::Neg for Quat {
    type Output = Quat;
    #[inline(always)]
    fn neg(self) -> Quat {
        Quat::new(-self.r(), -self.i(), -self.j(), -self.k())
    }
}

impl const std::ops::Mul for Quat {
    type Output = Quat;
    #[inline(always)]
    fn mul(self, rhs: Quat) -> Quat {
        Quat::new(
            self.r() * rhs.r() - self.i() * rhs.i() - self.j() * rhs.j() - self.k() * rhs.k(),
            self.r() * rhs.i() + self.i() * rhs.r() + self.j() * rhs.k() - self.k() * rhs.j(),
            self.r() * rhs.j() - self.i() * rhs.k() + self.j() * rhs.r() + self.k() * rhs.i(),
            self.r() * rhs.k() + self.i() * rhs.j() - self.j() * rhs.i() + self.k() * rhs.r(),
        )
    }
}

impl const std::ops::Mul<f32> for Quat {
    type Output = Quat;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Quat {
        Quat::new(
            self.r() * rhs,
            self.i() * rhs,
            self.j() * rhs,
            self.k() * rhs,
        )
    }
}

impl const std::ops::Mul<Vec3> for Quat {
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

impl const std::ops::Mul<Quat> for f32 {
    type Output = Quat;
    #[inline(always)]
    fn mul(self, rhs: Quat) -> Quat {
        rhs * self
    }
}

impl const From<Vec3> for Quat {
    #[inline(always)]
    fn from(vec: Vec3) -> Quat {
        Quat::new(0.0, vec.x(), vec.y(), vec.z())
    }
}

impl const From<f32> for Quat {
    #[inline(always)]
    fn from(r: f32) -> Quat {
        Quat::new(r, 0.0, 0.0, 0.0)
    }
}

impl Quat {
    pub const fn new(r: f32, i: f32, j: f32, k: f32) -> Quat {
        Quat { r, i, j, k }
    }

    pub const ONE: Quat = Quat::new(1.0, 0.0, 0.0, 0.0);

    pub fn rotation(axis: Vec3, angle: f32) -> Quat {
        let hf_angle = angle / 2.0;
        hf_angle.cos() + axis * hf_angle.sin()
    }

    pub const fn conj(self) -> Quat {
        Quat::new(self.r(), -self.i(), -self.j(), -self.k())
    }

    #[inline]
    pub const fn sq_mag(self) -> f32 {
        self.r() * self.r() + self.i() * self.i() + self.j() * self.j() + self.k() * self.k()
    }

    #[inline]
    pub fn mag(self) -> f32 {
        self.sq_mag().sqrt()
    }

    #[inline]
    pub const fn r(self) -> f32 {
        self.r
    }

    #[inline]
    pub const fn i(self) -> f32 {
        self.i
    }

    #[inline]
    pub const fn j(self) -> f32 {
        self.j
    }

    #[inline]
    pub const fn k(self) -> f32 {
        self.k
    }
}

#[cfg(test)]
mod vec3_tests {
    use super::*;

    #[test]
    fn new() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a.x(), 1.0);
        assert_eq!(a.y(), 2.0);
        assert_eq!(a.z(), 3.0);
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
        assert_eq!(b, 14.0);
    }

    #[test]
    fn mag() {
        let a: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let b: f32 = a.mag();
        assert_eq!(b, 14.0f32.sqrt());
    }

    #[test]
    fn rotate() {
        let v: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        let rot: Quat = Quat::rotation(Vec3::K, std::f32::consts::FRAC_PI_2);
        let v_new: Vec3 = v.rotate(rot);
        assert!((v_new - Vec3::new(-1.0, 1.0, 1.0)).sq_mag() < 1e-10);
    }
}

#[cfg(test)]
mod quat_tests {
    use super::*;

    #[test]
    fn new() {
        let a: Quat = Quat::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(a.r(), 1.0);
        assert_eq!(a.i(), 2.0);
        assert_eq!(a.j(), 3.0);
        assert_eq!(a.k(), 4.0);
    }

    #[test]
    fn add_quat() {
        let a: Quat = Quat::new(1.0, 1.0, 1.0, 1.0);
        let b: Quat = Quat::new(0.5, 1.0, 0.5, 1.0);
        let c: Quat = a + b;
        assert_eq!(c, Quat::new(1.5, 2.0, 1.5, 2.0));
    }

    #[test]
    fn add_f32() {
        let a: Quat = Quat::new(1.0, 0.5, 1.0, 2.0);
        let b: f32 = 10.0;
        let c: Quat = a + b;
        assert_eq!(c, Quat::new(11.0, 0.5, 1.0, 2.0));
    }

    #[test]
    fn sub() {
        let a: Quat = Quat::new(1.0, 1.0, 1.0, 1.0);
        let b: Quat = Quat::new(0.5, 1.0, 0.5, 1.0);
        let c: Quat = a - b;
        assert_eq!(c, Quat::new(0.5, 0.0, 0.5, 0.0));
    }

    #[test]
    fn neg() {
        let a: Quat = Quat::new(1.0, 1.0, 1.0, 1.0);
        let b: Quat = -a;
        assert_eq!(b, Quat::new(-1.0, -1.0, -1.0, -1.0));
    }

    #[test]
    fn mul_quat() {
        let a: Quat = Quat::new(1.0, 1.0, 1.0, 1.0);
        let b: Quat = Quat::new(0.5, 1.0, 0.5, 1.0);
        let c: Quat = a * b;
        assert_eq!(c, Quat::new(-2.0, 2.0, 1.0, 1.0));
    }

    #[test]
    fn mul_f32() {
        let a: Quat = Quat::new(0.5, 1.0, 0.5, 1.0);
        let b: f32 = 2.0;
        let c: Quat = a * b;
        assert_eq!(c, Quat::new(1.0, 2.0, 1.0, 2.0));
    }

    #[test]
    fn sq_mag() {
        let a: Quat = Quat::new(1.0, 2.0, 3.0, 4.0);
        let b: f32 = a.sq_mag();
        assert_eq!(b, 30.0);
    }

    #[test]
    fn mag() {
        let a: Quat = Quat::new(1.0, 2.0, 3.0, 4.0);
        let b: f32 = a.mag();
        assert_eq!(b, 30.0f32.sqrt());
    }
}
