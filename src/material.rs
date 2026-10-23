//! Материал поверхности (модель освещения Блинна–Фонга + отражения).

use crate::vec3::{Color, Vec3};

/// Шахматная раскраска (удобно для плоскости: видно перспективу и тени).
#[derive(Debug, Clone, Copy)]
pub struct Checker {
    /// Второй цвет клетки (первый — `Material::color`).
    pub color: Color,
    /// Размер одной клетки в единицах сцены.
    pub scale: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    /// Базовый цвет (альбедо).
    pub color: Color,
    /// Доля фонового (ambient) освещения.
    pub ambient: f64,
    /// Коэффициент диффузного (ламбертова) отражения.
    pub diffuse: f64,
    /// Коэффициент бликов.
    pub specular: f64,
    /// Резкость блика: чем больше, тем меньше пятно.
    pub shininess: f64,
    /// Зеркальность 0.0 (матовый) ..= 1.0 (идеальное зеркало).
    pub reflectivity: f64,
    /// Опциональная шахматная текстура.
    pub checker: Option<Checker>,
}

impl Material {
    /// Матовый материал с умеренным бликом.
    pub fn new(color: Color) -> Self {
        Material {
            color,
            ambient: 0.12,
            diffuse: 0.9,
            specular: 0.3,
            shininess: 32.0,
            reflectivity: 0.0,
            checker: None,
        }
    }

    pub fn with_specular(mut self, specular: f64, shininess: f64) -> Self {
        self.specular = specular;
        self.shininess = shininess;
        self
    }

    pub fn with_reflectivity(mut self, reflectivity: f64) -> Self {
        self.reflectivity = reflectivity.clamp(0.0, 1.0);
        self
    }

    pub fn with_checker(mut self, second: Color, scale: f64) -> Self {
        self.checker = Some(Checker { color: second, scale });
        self
    }

    /// Цвет поверхности в точке `p` (учитывает шахматную текстуру).
    pub fn color_at(&self, p: Vec3) -> Color {
        match self.checker {
            Some(ch) => {
                let i = (p.x / ch.scale).floor() as i64 + (p.z / ch.scale).floor() as i64;
                if i.rem_euclid(2) == 0 {
                    self.color
                } else {
                    ch.color
                }
            }
            None => self.color,
        }
    }
}
