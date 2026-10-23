//! Точечный источник света.

use crate::vec3::{Color, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vec3,
    pub color: Color,
    /// Яркость: 1.0 — «нормально», 0.3 — тусклый, 2.0 — очень яркий.
    pub intensity: f64,
}

impl Light {
    /// Белый свет заданной яркости.
    pub fn new(position: Vec3, intensity: f64) -> Self {
        Light {
            position,
            color: Vec3::ONE,
            intensity,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Итоговая энергия света (цвет * яркость).
    #[inline]
    pub fn radiance(&self) -> Color {
        self.color * self.intensity
    }
}
