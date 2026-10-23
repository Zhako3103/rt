//! Готовые сцены. Здесь удобно экспериментировать: меняйте позиции,
//! размеры, материалы и яркость света и пересобирайте проект.

use crate::light::Light;
use crate::material::Material;
use crate::objects::{Cube, Cylinder, Plane, Sphere};
use crate::scene::Scene;
use crate::vec3::Vec3;

/// Параметры точки обзора (по ним строится `Camera`).
#[derive(Debug, Clone, Copy)]
pub struct View {
    pub from: Vec3,
    pub at: Vec3,
    pub fov: f64,
}

/// (имя, номер, описание) — для `--list`.
pub const SCENE_LIST: &[(&str, &str, &str)] = &[
    ("sphere", "1", "одна сфера"),
    ("plane-cube", "2", "плоскость и куб, свет тусклее (intensity 0.4)"),
    ("all", "3", "сфера, куб, цилиндр и плоскость, вид спереди"),
    ("all-side", "4", "та же сцена, другой ракурс камеры"),
    ("mirror", "5", "бонус: зеркальная и глянцевая сферы, как на референсе"),
];

#[inline]
fn v(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

/// Собрать сцену по имени или номеру.
pub fn build(name: &str) -> Option<(Scene, View)> {
    match name {
        "1" | "sphere" => Some(sphere_only()),
        "2" | "plane-cube" => Some(plane_and_cube()),
        "3" | "all" => Some(all_objects(false)),
        "4" | "all-side" => Some(all_objects(true)),
        "5" | "mirror" => Some(mirror_demo()),
        _ => None,
    }
}

/// Сцена 1: одна сфера на фоне неба.
fn sphere_only() -> (Scene, View) {
    let mut scene = Scene::new();
    scene.add(Sphere::new(
        v(0.0, 0.0, 0.0),
        1.2,
        Material::new(v(0.9, 0.25, 0.15)).with_specular(0.6, 64.0),
    ));
    scene.add_light(Light::new(v(-3.0, 4.0, 5.0), 1.0));
    let view = View { from: v(0.0, 0.5, 5.0), at: v(0.0, 0.0, 0.0), fov: 40.0 };
    (scene, view)
}

/// Сцена 2: плоскость + куб; единственный источник заметно тусклее, чем в сцене 3.
fn plane_and_cube() -> (Scene, View) {
    let mut scene = Scene::new();
    scene.add(Plane::horizontal(
        0.0,
        Material::new(v(0.8, 0.8, 0.8))
            .with_checker(v(0.3, 0.3, 0.35), 1.0)
            .with_specular(0.0, 1.0),
    ));
    scene.add(
        Cube::cube(v(0.0, 0.75, 0.0), 1.5, Material::new(v(0.2, 0.4, 0.85)))
            .with_rotation_y(30.0),
    );
    scene.add_light(Light::new(v(-3.0, 5.0, 4.0), 0.4)); // <- яркость света
    let view = View { from: v(4.0, 3.0, 6.0), at: v(0.0, 0.6, 0.0), fov: 40.0 };
    (scene, view)
}

/// Сцены 3 и 4: все примитивы сразу. `side` выбирает ракурс камеры.
fn all_objects(side: bool) -> (Scene, View) {
    let mut scene = Scene::new();

    // Плоскость-«пол» с шахматной текстурой.
    scene.add(Plane::horizontal(
        0.0,
        Material::new(v(0.85, 0.85, 0.85))
            .with_checker(v(0.25, 0.25, 0.3), 1.0)
            .with_specular(0.0, 1.0),
    ));

    // Слегка зеркальная красная сфера (включено отражение).
    scene.add(Sphere::new(
        v(-2.4, 1.0, 0.0),
        1.0,
        Material::new(v(0.85, 0.15, 0.12))
            .with_specular(0.6, 64.0)
            .with_reflectivity(0.25),
    ));

    // Повёрнутый синий куб.
    scene.add(
        Cube::cube(v(0.0, 0.75, 0.0), 1.5, Material::new(v(0.2, 0.4, 0.85)))
            .with_rotation_y(25.0),
    );

    // Зелёный цилиндр: радиус 0.8, высота 2.
    scene.add(Cylinder::new(
        v(2.4, 1.0, 0.0),
        0.8,
        2.0,
        Material::new(v(0.2, 0.7, 0.3)).with_specular(0.4, 48.0),
    ));

    // Основной свет и слабая подсветка с другой стороны.
    scene.add_light(Light::new(v(-4.0, 6.0, 6.0), 1.0));
    scene.add_light(Light::new(v(6.0, 3.0, 4.0), 0.25));

    let view = if side {
        View { from: v(7.0, 4.0, 5.0), at: v(0.0, 0.8, 0.0), fov: 40.0 }
    } else {
        View { from: v(0.0, 3.0, 9.0), at: v(0.0, 0.8, 0.0), fov: 40.0 }
    };
    (scene, view)
}

/// Сцена 5 (бонус): повторяет референсное изображение.
fn mirror_demo() -> (Scene, View) {
    let mut scene = Scene::new();
    scene.add(Plane::horizontal(
        0.0,
        Material::new(v(0.45, 0.52, 0.62)).with_specular(0.0, 1.0),
    ));
    // Зеркальная сфера.
    scene.add(Sphere::new(
        v(-1.6, 1.0, -1.0),
        1.0,
        Material::new(v(0.9, 0.9, 0.9))
            .with_specular(0.8, 128.0)
            .with_reflectivity(0.85),
    ));
    // Оливковая глянцевая сфера.
    scene.add(Sphere::new(
        v(1.4, 1.2, 0.5),
        1.2,
        Material::new(v(0.55, 0.62, 0.2))
            .with_specular(0.4, 40.0)
            .with_reflectivity(0.1),
    ));
    scene.add_light(Light::new(v(-3.0, 8.0, 6.0), 1.0));
    let view = View { from: v(0.0, 1.4, 7.0), at: v(0.0, 0.9, 0.0), fov: 40.0 };
    (scene, view)
}
