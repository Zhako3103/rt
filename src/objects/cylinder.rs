//! Конечный цилиндр с осью, параллельной Y, и двумя плоскими крышками.

use super::{Hit, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Cylinder {
    /// Центр цилиндра (середина оси).
    pub center: Vec3,
    pub radius: f64,
    /// Полная высота вдоль оси Y.
    pub height: f64,
    pub material: Material,
}

impl Cylinder {
    pub fn new(center: Vec3, radius: f64, height: f64, material: Material) -> Self {
        Cylinder { center, radius, height, material }
    }
}

impl Hittable for Cylinder {
    /// Боковая поверхность: x² + z² = r² (в локальных координатах центра).
    /// Подставляем луч, учитывая только x и z:
    ///   a = Dx² + Dz²,  b = Ox·Dx + Oz·Dz,  c = Ox² + Oz² - r²
    ///   a t² + 2 b t + c = 0.
    /// Корень годится, если высота точки |y| <= height/2.
    ///
    /// Крышки: плоскости y = ±height/2; точка годится, если x² + z² <= r².
    /// Из всех кандидатов выбираем минимальное t.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit<'_>> {
        let o = ray.origin - self.center;
        let d = ray.direction;
        let half_h = self.height * 0.5;
        let r2 = self.radius * self.radius;

        let mut best_t = t_max;
        let mut best_n: Option<Vec3> = None;

        // --- боковая поверхность ---
        let a = d.x * d.x + d.z * d.z;
        if a > 1e-12 {
            let b = o.x * d.x + o.z * d.z;
            let c = o.x * o.x + o.z * o.z - r2;
            let disc = b * b - a * c;
            if disc >= 0.0 {
                let sq = disc.sqrt();
                for t in [(-b - sq) / a, (-b + sq) / a] {
                    if t >= t_min && t < best_t {
                        let y = o.y + t * d.y;
                        if y.abs() <= half_h {
                            best_t = t;
                            // Нормаль боковой стенки горизонтальна и идёт от оси.
                            best_n = Some(Vec3::new(o.x + t * d.x, 0.0, o.z + t * d.z) / self.radius);
                        }
                    }
                }
            }
        }

        // --- крышки (нижняя y = -h/2, верхняя y = +h/2) ---
        if d.y.abs() > 1e-12 {
            for sign in [-1.0f64, 1.0] {
                let t = (sign * half_h - o.y) / d.y;
                if t >= t_min && t < best_t {
                    let x = o.x + t * d.x;
                    let z = o.z + t * d.z;
                    if x * x + z * z <= r2 {
                        best_t = t;
                        best_n = Some(Vec3::new(0.0, sign, 0.0));
                    }
                }
            }
        }

        best_n.map(|normal| Hit {
            t: best_t,
            point: ray.at(best_t),
            normal,
            material: &self.material,
        })
    }
}
