//! Куб / параллелепипед. Размеры по осям задаются независимо,
//! плюс возможен поворот вокруг вертикальной оси Y.

use super::{Hit, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Cube {
    pub center: Vec3,
    /// Полные длины рёбер по осям X, Y, Z.
    pub size: Vec3,
    /// Поворот вокруг оси Y, радианы.
    angle_y: f64,
    pub material: Material,
}

/// Поворот вектора вокруг оси Y на угол `a`.
#[inline]
fn rotate_y(v: Vec3, a: f64) -> Vec3 {
    let (s, c) = a.sin_cos();
    Vec3::new(v.x * c + v.z * s, v.y, -v.x * s + v.z * c)
}

impl Cube {
    /// Параллелепипед с произвольными размерами.
    pub fn new(center: Vec3, size: Vec3, material: Material) -> Self {
        Cube { center, size, angle_y: 0.0, material }
    }

    /// Настоящий куб с ребром `edge`.
    pub fn cube(center: Vec3, edge: f64, material: Material) -> Self {
        Cube::new(center, Vec3::new(edge, edge, edge), material)
    }

    /// Повернуть вокруг оси Y (в градусах).
    pub fn with_rotation_y(mut self, degrees: f64) -> Self {
        self.angle_y = degrees.to_radians();
        self
    }
}

impl Hittable for Cube {
    /// Метод «плит» (slab method). Куб — пересечение трёх пар параллельных
    /// плоскостей. Для каждой оси находим интервал [t0, t1], в котором луч
    /// находится между плоскостями; пересечение интервалов трёх осей —
    /// отрезок луча внутри куба: вход = max(t0), выход = min(t1).
    /// Если вход > выход — промах.
    ///
    /// Чтобы поддержать поворот, переносим луч в локальную систему куба
    /// (сдвиг в центр + обратный поворот): там куб осево-выровненный.
    /// Поворот сохраняет длины, поэтому `t` одинаково в обеих системах.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_>> {
        let o = rotate_y(ray.origin - self.center, -self.angle_y);
        let d = rotate_y(ray.direction, -self.angle_y);
        let half = self.size * 0.5;

        let mut t_near = f64::NEG_INFINITY;
        let mut t_far = f64::INFINITY;
        let mut axis_near = 0usize;
        let mut axis_far = 0usize;

        for i in 0..3 {
            if d[i].abs() < 1e-12 {
                // Луч параллелен плитам: либо всегда внутри, либо никогда.
                if o[i].abs() > half[i] {
                    return None;
                }
                continue;
            }
            let inv = 1.0 / d[i];
            let mut t0 = (-half[i] - o[i]) * inv;
            let mut t1 = (half[i] - o[i]) * inv;
            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }
            if t0 > t_near {
                t_near = t0;
                axis_near = i;
            }
            if t1 < t_far {
                t_far = t1;
                axis_far = i;
            }
            if t_near > t_far {
                return None;
            }
        }

        // Входная точка, а если она вне диапазона (камера внутри куба) — выходная.
        let (t, axis, sign) = if t_near >= t_min && t_near <= t_max {
            // Вошли через грань, нормаль которой направлена против луча.
            (t_near, axis_near, -d[axis_near].signum())
        } else if t_far >= t_min && t_far <= t_max {
            (t_far, axis_far, d[axis_far].signum())
        } else {
            return None;
        };

        // Нормаль — единичный вектор вдоль оси грани (в локальных координатах),
        // затем поворачиваем обратно в мировые.
        let mut n = [0.0f64; 3];
        n[axis] = sign;
        let normal = rotate_y(Vec3::new(n[0], n[1], n[2]), self.angle_y);

        Some(Hit {
            t,
            point: ray.at(t),
            normal,
            material: &self.material,
        })
    }
}
