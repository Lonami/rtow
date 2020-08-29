use crate::{Color, Hit, Ray, Vec3};

pub trait Material {
    /// Returns the resulting ray and attenuation color.
    // Accept a `Hit` to avoid passing a lot of parameters.
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    // Alternatively, we could scatter only with probability p and have attenuation be albedo / p.
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let scatter_dir = hit.normal + Vec3::new_random_unit();
        let scattered = Ray::new(hit.point, scatter_dir);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}
