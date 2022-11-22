use crate::{
    math::Vec3,
    world::{Color, Object, Transform, World},
};

pub struct Camera {
    pub transform: Transform,
    pub px_per_unit: f32,
    pub focal_length: f32,
}

impl Camera {
    pub fn get_px(&self, world: &World, x: f32, y: f32) -> Color {
        let ray = Vec3::new(
            x / self.px_per_unit,
            self.focal_length,
            -y / self.px_per_unit,
        )
        .rotate(self.transform.rotation);

        Self::raycast(self.transform.position, ray, world, true)
            .map_or(Color::BLACK, |hit| hit.color)
    }

    fn raycast(base: Vec3, ray: Vec3, world: &World, shadows: bool) -> Option<RcHit> {
        let mut hit = world
            .objects
            .iter()
            .filter_map(|obj| Self::calc_raycast(base, ray, obj))
            .min_by(|a, b| a.t.total_cmp(&b.t))?;
        let color = &mut hit.color;

        let coord = base + ray * hit.t;
        let light_vec = (world.light - coord).normalize();
        if shadows
            && world.objects.iter().any(|obj| {
                // Check that the raycast hit is not the suface itself.
                // `f32::EPSILON` is too small and creates visual artifacts.
                Self::calc_raycast(coord, light_vec, obj).is_some_and(|hit| hit.t > 1e-4)
            })
        {
            *color = Color::BLACK
        } else {
            let illumination = light_vec.dot(hit.normal).max(0.0);
            *color = *color * illumination;
        }

        Some(hit)
    }

    fn calc_raycast(base: Vec3, ray: Vec3, obj: &Object) -> Option<RcHit> {
        match *obj {
            Object::Sphere(center, r, color) => {
                Self::calc_sphere_raycast(base, ray, (center, r, color))
            }
            Object::Triangle(p1, p2, p3, color) => {
                Self::calc_tri_raycast(base, ray, (p1, p2, p3, color))
            }
        }
    }

    fn calc_tri_raycast(
        base: Vec3,
        ray: Vec3,
        (p1, p2, p3, color): (Vec3, Vec3, Vec3, Color),
    ) -> Option<RcHit> {
        // Check if within plane
        let v1 = p2 - p1;
        let v2 = p3 - p1;
        let dist = base - p1;
        let cross = v1.cross(v2);
        let t = -cross.dot(dist) / cross.dot(ray);

        if !t.is_finite() || t.is_sign_negative() {
            return None;
        }

        // Check if within tetrahedron
        let [(a, d, g), (b, e, h), (c, f, i)] =
            [p1, p2, p3].map(|p| p - base).map(|v| (v.x, v.y, v.z));

        let ei_fh = e * i - f * h;
        let fg_di = f * g - d * i;
        let dh_eg = d * h - e * g;
        let det_neg = (a * ei_fh + b * fg_di + c * dh_eg).is_sign_negative();
        if ![
            ray.x * ei_fh + ray.y * (c * h - b * i) + ray.z * (b * f - c * e),
            ray.x * fg_di + ray.y * (a * i - c * g) + ray.z * (c * d - a * f),
            ray.x * dh_eg + ray.y * (b * g - a * h) + ray.z * (a * e - b * d),
        ]
        .iter()
        .all(|n| n.is_sign_positive() ^ det_neg)
        {
            return None;
        }

        let normal = cross.normalize();

        Some(RcHit::new(color, t, normal))
    }

    fn calc_sphere_raycast(
        base: Vec3,
        ray: Vec3,
        (center, r, color): (Vec3, f32, Color),
    ) -> Option<RcHit> {
        let dist = center - base;
        let a = ray.sq_mag();
        // SAFETY: `a` will always be positive; we let LLVM know so this can be optimized.
        unsafe {
            std::intrinsics::assume(a >= 0.0);
        }

        let b = ray.dot(dist);
        let c = dist.sq_mag() - r.powi(2);

        let discriminant = b.powi(2) - a * c;
        if discriminant.is_sign_negative() || discriminant.is_subnormal() {
            return None;
        }

        let sqrt_term = discriminant.sqrt();

        let t = [b + sqrt_term, b - sqrt_term]
            .into_iter()
            .filter(|n| n.is_sign_positive())
            .min_by(f32::total_cmp)
            // `a` will never be negative as it is the result of the `sq_mag` of a `Vec3`.
            // As such, dividing by `a` does not have a chance of flipping the signs of the rest of the `t` calculation.
            .map(|n| n / a)?;

        let coord = base + ray * t;
        let normal = (coord - center).normalize();

        Some(RcHit::new(color, t, normal))
    }
}

struct RcHit {
    color: Color,
    t: f32,
    normal: Vec3,
}

impl RcHit {
    fn new(color: Color, t: f32, normal: Vec3) -> Self {
        Self { color, t, normal }
    }
}
