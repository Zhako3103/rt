//! Сфера: |P - C|² = r².

use super::{Hit, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    /// Подставляем P(t) = O + tD в уравнение сферы, получаем квадратное
    /// уравнение a t² + 2 b t + c = 0, где
    ///   a = D·D, b = D·(O-C), c = |O-C|² - r².
    /// Дискриминант b² - a c < 0 — луч промахнулся.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let disc = half_b * half_b - a * c;
        if disc < 0.0 {
            return None;
        }
        let sq = disc.sqrt();

        // Берём ближайший корень в допустимом диапазоне
        // (дальний корень нужен, когда камера внутри сферы).
        let mut t = (-half_b - sq) / a;
        if t < t_min || t > t_max {
            t = (-half_b + sq) / a;
            if t < t_min || t > t_max {
                return None;
            }
        }

        let point = ray.at(t);
        // Нормаль сферы — радиус-вектор из центра в точку, нормированный.
        let normal = (point - self.center) / self.radius;
        Some(Hit { t, point, normal, material: &self.material })
    }
}
