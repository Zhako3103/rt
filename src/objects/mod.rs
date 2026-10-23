//! Геометрические примитивы и общий интерфейс пересечения с лучом.

pub mod cube;
pub mod cylinder;
pub mod plane;
pub mod sphere;

pub use cube::Cube;
pub use cylinder::Cylinder;
pub use plane::Plane;
pub use sphere::Sphere;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

/// Информация о ближайшем пересечении луча с объектом.
pub struct Hit<'a> {
    /// Расстояние вдоль луча.
    pub t: f64,
    /// Точка пересечения в мировых координатах.
    pub point: Vec3,
    /// Единичная нормаль (может смотреть как наружу, так и внутрь:
    /// рендерер при необходимости разворачивает её против луча).
    pub normal: Vec3,
    pub material: &'a Material,
}

/// Всё, что может быть пересечено лучом.
/// `Send + Sync` нужны, чтобы сцену можно было читать из нескольких потоков.
pub trait Hittable: Send + Sync {
    /// Ближайшее пересечение с параметром `t` в диапазоне `[t_min, t_max]`.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_>>;
}
