//! Сцена: объекты, источники света и фон (небо).

use crate::light::Light;
use crate::objects::{Hit, Hittable};
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub lights: Vec<Light>,
    /// Цвет и сила общего фонового освещения (умножается на `Material::ambient`).
    pub ambient: Color,
    /// Градиент неба: у горизонта и в зените.
    pub sky_horizon: Color,
    pub sky_zenith: Color,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            ambient: Vec3::ONE,
            sky_horizon: Vec3::new(0.85, 0.92, 1.0),
            sky_zenith: Vec3::new(0.45, 0.65, 1.0),
        }
    }

    pub fn add<O: Hittable + 'static>(&mut self, object: O) -> &mut Self {
        self.objects.push(Box::new(object));
        self
    }

    pub fn add_light(&mut self, light: Light) -> &mut Self {
        self.lights.push(light);
        self
    }

    /// Ближайшее пересечение луча со всей сценой.
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_>> {
        let mut closest = t_max;
        let mut result = None;
        for obj in &self.objects {
            if let Some(h) = obj.hit(ray, t_min, closest) {
                closest = h.t;
                result = Some(h);
            }
        }
        result
    }

    /// Есть ли хоть что-то на пути теневого луча (для теней достаточно
    /// любого пересечения — ближайшее искать не нужно).
    pub fn occluded(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        self.objects.iter().any(|o| o.hit(ray, t_min, t_max).is_some())
    }

    /// Цвет неба для луча, ушедшего в никуда.
    pub fn background(&self, direction: Vec3) -> Color {
        let t = direction.y.clamp(0.0, 1.0).sqrt();
        self.sky_horizon.lerp(self.sky_zenith, t)
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
