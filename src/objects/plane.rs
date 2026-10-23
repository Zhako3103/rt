//! Бесконечная плоскость: (P - P0)·N = 0.

use super::{Hit, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Plane {
    /// Любая точка на плоскости (задаёт положение).
    pub point: Vec3,
    /// Единичная нормаль (задаёт ориентацию).
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Material) -> Self {
        Plane {
            point,
            normal: normal.normalized(),
            material,
        }
    }

    /// Горизонтальная плоскость на высоте `y` (нормаль смотрит вверх).
    pub fn horizontal(y: f64, material: Material) -> Self {
        Plane::new(Vec3::new(0.0, y, 0.0), Vec3::UP, material)
    }
}

impl Hittable for Plane {
    /// Подставляем P(t) = O + tD в (P - P0)·N = 0:
    ///   t = (P0 - O)·N / (D·N).
    /// Если D·N ≈ 0, луч параллелен плоскости.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_>> {
        let denom = ray.direction.dot(self.normal);
        if denom.abs() < 1e-9 {
            return None;
        }
        let t = (self.point - ray.origin).dot(self.normal) / denom;
        if t < t_min || t > t_max {
            return None;
        }
        Some(Hit {
            t,
            point: ray.at(t),
            normal: self.normal,
            material: &self.material,
        })
    }
}
