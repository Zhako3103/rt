//! Рендерер: трассировка лучей, освещение по Блинну–Фонгу, тени, отражения.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use crate::camera::Camera;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::vec3::{Color, Vec3};

/// Смещение начала вторичных лучей, чтобы поверхность не затеняла саму себя
/// из-за ошибок округления («shadow acne»).
const EPS: f64 = 1e-4;

#[derive(Debug, Clone, Copy)]
pub struct RenderSettings {
    pub width: usize,
    pub height: usize,
    /// Лучей на пиксель (>1 включает сглаживание краёв).
    pub samples: usize,
    /// Максимальная глубина рекурсии отражений.
    pub max_depth: u32,
    /// Число потоков.
    pub threads: usize,
}

/// Крошечный генератор псевдослучайных чисел (xorshift64) — чтобы не
/// тянуть внешний крейт `rand`. Нужен только для субпиксельного джиттера.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed.wrapping_add(1).wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    /// Число в [0, 1).
    fn next_f64(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

/// Рекурсивно вычисляет цвет, который «видит» луч.
fn trace(scene: &Scene, ray: &Ray, depth: u32, max_depth: u32) -> Color {
    // 1. Ближайшее пересечение; если его нет — виден фон.
    let hit = match scene.hit(ray, EPS, f64::INFINITY) {
        Some(h) => h,
        None => return scene.background(ray.direction),
    };

    let mat = hit.material;
    let albedo = mat.color_at(hit.point);

    // 2. Нормаль должна смотреть навстречу лучу (важно для плоскости,
    //    которую можно увидеть с обеих сторон, и для камеры внутри тела).
    let n = if hit.normal.dot(ray.direction) > 0.0 {
        -hit.normal
    } else {
        hit.normal
    };

    // 3. Фоновое освещение.
    let mut color = albedo * scene.ambient * mat.ambient;

    // 4. Прямое освещение от каждого источника.
    for light in &scene.lights {
        let to_light = light.position - hit.point;
        let dist = to_light.length();
        let l = to_light / dist;

        // Закон Ламберта: яркость ∝ cos угла между нормалью и направлением на свет.
        let n_dot_l = n.dot(l);
        if n_dot_l <= 0.0 {
            continue; // свет «позади» поверхности
        }

        // Теневой луч: если до источника что-то мешает, свет не доходит.
        let shadow_ray = Ray::new(hit.point + n * EPS, l);
        if scene.occluded(&shadow_ray, EPS, dist - EPS) {
            continue;
        }

        let radiance = light.radiance();

        // Диффузная составляющая.
        color += albedo * radiance * (mat.diffuse * n_dot_l);

        // Блик Блинна–Фонга: полувектор между направлениями на свет и на камеру.
        if mat.specular > 0.0 {
            let h = (l - ray.direction).normalized();
            let spec = n.dot(h).max(0.0).powf(mat.shininess) * mat.specular;
            color += radiance * spec;
        }
    }

    // 5. Отражения: смешиваем собственный цвет с цветом, увиденным в «зеркале».
    if mat.reflectivity > 0.0 && depth < max_depth {
        let refl_dir = ray.direction.reflect(n);
        let refl_ray = Ray::new(hit.point + n * EPS, refl_dir);
        let refl_color = trace(scene, &refl_ray, depth + 1, max_depth);
        color = color * (1.0 - mat.reflectivity) + refl_color * mat.reflectivity;
    }

    color
}

/// Преобразует линейный цвет в 3 байта: ограничение + гамма-коррекция (γ = 2).
#[inline]
fn to_bytes(c: Color) -> [u8; 3] {
    let c = c.clamp01();
    [
        (c.x.sqrt() * 255.0 + 0.5) as u8,
        (c.y.sqrt() * 255.0 + 0.5) as u8,
        (c.z.sqrt() * 255.0 + 0.5) as u8,
    ]
}

/// Рендер одной строки изображения (y = 0 — верхняя).
fn render_row(scene: &Scene, camera: &Camera, s: &RenderSettings, y: usize) -> Vec<u8> {
    let mut row = Vec::with_capacity(s.width * 3);
    let mut rng = Rng::new(y as u64);
    let samples = s.samples.max(1);

    for x in 0..s.width {
        let mut sum = Vec3::ZERO;
        for _ in 0..samples {
            // При одном луче целимся в центр пикселя, иначе — случайно внутри него.
            let (jx, jy) = if samples == 1 {
                (0.5, 0.5)
            } else {
                (rng.next_f64(), rng.next_f64())
            };
            let u = (x as f64 + jx) / s.width as f64;
            let v = 1.0 - (y as f64 + jy) / s.height as f64; // ось Y экрана направлена вниз
            let ray = camera.get_ray(u, v);
            sum += trace(scene, &ray, 0, s.max_depth);
        }
        row.extend_from_slice(&to_bytes(sum / samples as f64));
    }
    row
}

/// Рендерит всю сцену; возвращает плоский RGB-буфер (width * height * 3).
/// Строки раздаются потокам динамически через атомарный счётчик.
pub fn render(scene: &Scene, camera: &Camera, settings: &RenderSettings) -> Vec<u8> {
    let (w, h) = (settings.width, settings.height);
    let threads = settings.threads.max(1);
    let next_row = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let next_row = &next_row;
    let done = &done;

    let results: Vec<Vec<(usize, Vec<u8>)>> = thread::scope(|scope| {
        let mut handles = Vec::new();
        for _ in 0..threads {
            handles.push(scope.spawn(move || {
                let mut local = Vec::new();
                loop {
                    let y = next_row.fetch_add(1, Ordering::Relaxed);
                    if y >= h {
                        break;
                    }
                    local.push((y, render_row(scene, camera, settings, y)));
                    let n = done.fetch_add(1, Ordering::Relaxed) + 1;
                    if n % 25 == 0 || n == h {
                        eprint!("\rРендер: {}/{} строк ({}%)", n, h, n * 100 / h);
                    }
                }
                local
            }));
        }
        handles.into_iter().map(|j| j.join().unwrap()).collect()
    });
    eprintln!();

    let mut pixels = vec![0u8; w * h * 3];
    for (y, row) in results.into_iter().flatten() {
        pixels[y * w * 3..(y + 1) * w * 3].copy_from_slice(&row);
    }
    pixels
}
