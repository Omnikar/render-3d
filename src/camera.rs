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
        let ray = Vec3 {
            x: self.focal_length,
            y: -x / self.px_per_unit,
            z: -y / self.px_per_unit,
        }
        .rotate(self.transform.rotation);

        world
            .objects
            .iter()
            .filter_map(|p| self.raycast(ray, world.light, p))
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(color, _)| color)
            .unwrap_or(Color([0; 3]))
    }

    fn raycast(&self, ray: Vec3, light: Vec3, p: &Object) -> Option<(Color, f32)> {
        match p {
            Object::Sphere(center, r, color) => {
                self.sphere_raycast(ray, light, (*center, *r, *color))
            }
            Object::Triangle(p1, p2, p3, color) => {
                self.tri_raycast(ray, light, (*p1, *p2, *p3, *color))
            }
        }
    }

    fn tri_raycast(
        &self,
        ray: Vec3,
        light: Vec3,
        (p1, p2, p3, color): (Vec3, Vec3, Vec3, Color),
    ) -> Option<(Color, f32)> {
        // Check if within plane
        let v1 = p2 - p1;
        let v2 = p3 - p1;
        let dist = self.transform.position - p1;
        let cross = v1.cross(v2);
        let t = -cross.dot(dist) / cross.dot(ray);

        if !t.is_finite() || t.is_sign_negative() {
            return None;
        }

        // Check if within tetrahedron
        let [(a, d, g), (b, e, h), (c, f, i)] = [p1, p2, p3]
            .map(|p| p - self.transform.position)
            .map(|v| (v.x, v.y, v.z));

        let ei_fh = e * i - f * h;
        let fg_di = f * g - d * i;
        let dh_eg = d * h - e * g;
        let det_neg = (a * (ei_fh) + b * (fg_di) + c * (dh_eg)).is_sign_negative();
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
        // `a` will always be positive, let LLVM know so this can be optimized
        unsafe {
            std::intrinsics::assume(a >= 0.0);
        }

        let b = ray.dot(dist);
        let c = dist.sq_mag() - r.powi(2);

        let sqrt_term_inner = b.powi(2) - a * c;
        if (0.0 > sqrt_term_inner) || sqrt_term_inner.is_subnormal() {
            return None;
        }

        let sqrt_term = sqrt_term_inner.sqrt();

        let t = [(b + sqrt_term), (b - sqrt_term)]
            .into_iter()
            .filter(|n| n.is_sign_positive())
            .min_by(f32::total_cmp)
            // `a` will  never be negative as `a` is the result of the `sq_mag` of a `Vec3`
            .map(|n| n / a)?;

        let coord = self.transform.position + ray * t;
        let normal = (coord - center).normalize();
        let light_vec = (light - coord).normalize();
        let illumination = light_vec.dot(normal).max(0.0);
        let color = color * illumination;
        Some((color, t))
    }
}
