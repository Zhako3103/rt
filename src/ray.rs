//! Луч: P(t) = origin + t * direction, где direction — единичный вектор.

use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Направление нормализуется автоматически, поэтому параметр `t`
    /// в пересечениях равен реальному расстоянию вдоль луча.
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction: direction.normalized(),
        }
    }

    /// Точка на луче на расстоянии `t` от начала.
    #[inline]
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
