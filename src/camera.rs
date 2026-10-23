//! Камера-«дырочка» (pinhole): позиция, точка, на которую смотрим, угол обзора.

use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /// * `look_from` — положение камеры;
    /// * `look_at`   — точка, в которую направлена камера (задаёт направление взгляда);
    /// * `vup`       — вектор «вверх» (обычно (0,1,0));
    /// * `vfov_deg`  — вертикальный угол обзора в градусах;
    /// * `aspect`    — ширина / высота изображения.
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov_deg: f64, aspect: f64) -> Self {
        let theta = vfov_deg.to_radians();
        let viewport_h = 2.0 * (theta / 2.0).tan(); // высота кадра на расстоянии 1
        let viewport_w = aspect * viewport_h;

        // Ортонормированный базис камеры: w смотрит «назад», u — вправо, v — вверх.
        let w = (look_from - look_at).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);

        let horizontal = u * viewport_w;
        let vertical = v * viewport_h;
        let lower_left = look_from - horizontal * 0.5 - vertical * 0.5 - w;

        Camera {
            origin: look_from,
            lower_left,
            horizontal,
            vertical,
        }
    }

    /// Луч через точку кадра (s, t): s — слева направо, t — снизу вверх, оба в 0..=1.
    #[inline]
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let target = self.lower_left + self.horizontal * s + self.vertical * t;
        Ray::new(self.origin, target - self.origin)
    }
}
