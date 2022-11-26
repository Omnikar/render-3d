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
        if shadows && {
            let max_t_sq = (world.light - coord).sq_mag();
            world.objects.iter().any(|obj| {
                // Check that the raycast hit is not the suface itself.
                // `f32::EPSILON` is too small and creates visual artifacts.
                Self::calc_raycast(coord, light_vec, obj)
                    .is_some_and(|hit| hit.t > 1e-4 && hit.t * hit.t < max_t_sq)
            })
        } {
            *color = Color::BLACK;
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
        let [(x1, x2, x3), (y1, y2, y3), (z1, z2, z3)] =
            [p1, p2, p3].map(|p| p - base).map(|v| (v.x, v.y, v.z));

        let y2z3_z2y3 = y2 * z3 - z2 * y3;
        let z2x3_x2z3 = z2 * x3 - x2 * z3;
        let x2y3_y2x3 = x2 * y3 - y2 * x3;
        let det_neg = (x1 * y2z3_z2y3 + y1 * z2x3_x2z3 + z1 * x2y3_y2x3).is_sign_negative();
        if ![
            ray.x * y2z3_z2y3 + ray.y * (z1 * y3 - y1 * z3) + ray.z * (y1 * z2 - z1 * y2),
            ray.x * z2x3_x2z3 + ray.y * (x1 * z3 - z1 * x3) + ray.z * (z1 * x2 - x1 * z2),
            ray.x * x2y3_y2x3 + ray.y * (y1 * x3 - x1 * y3) + ray.z * (x1 * y2 - y1 * x2),
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
        let ray_sqmag = ray.sq_mag();
        // SAFETY: `ray_sqmag` will always be positive; we let LLVM know so this can be optimized.
        unsafe {
            std::intrinsics::assume(ray_sqmag >= 0.0);
        }

        let dot = ray.dot(dist);
        let d_r_sqmag = dist.sq_mag() - r * r;

        let discriminant = dot * dot - ray_sqmag * d_r_sqmag;
        if discriminant.is_sign_negative() || discriminant.is_subnormal() {
            return None;
        }

        let sqrt_term = discriminant.sqrt();

        let t = [dot + sqrt_term, dot - sqrt_term]
            .into_iter()
            .filter(|n| n.is_sign_positive())
            .min_by(f32::total_cmp)
            // `ray_sqmag` will never be negative as it is the result of the `sq_mag` of a `Vec3`.
            // As such, dividing by `ray_sqmag` does not have a chance of flipping the signs of
            // the rest of the `t` calculation.
            .map(|n| n / ray_sqmag)?;

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
    const fn new(color: Color, t: f32, normal: Vec3) -> Self {
        Self { color, t, normal }
    }
}
