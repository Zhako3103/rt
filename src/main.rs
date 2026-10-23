//! Трассировщик лучей на чистом Rust. Результат — PPM (P3) в stdout или в файл.
//!
//! Запуск: `cargo run --release -- --scene 3 > output.ppm`

#![allow(dead_code)]

mod camera;
mod image;
mod light;
mod material;
mod objects;
mod ray;
mod renderer;
mod scene;
mod scenes;
mod vec3;

use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::process;
use std::time::Instant;

use camera::Camera;
use renderer::RenderSettings;
use vec3::Vec3;

const HELP: &str = "\
Трассировщик лучей (вывод: PPM P3)

ИСПОЛЬЗОВАНИЕ:
    cargo run --release -- [ОПЦИИ] > output.ppm

ОПЦИИ:
    -s, --scene <имя|номер>   сцена: sphere(1), plane-cube(2), all(3), all-side(4), mirror(5) [по умолчанию: 3]
    -w, --width  <px>         ширина изображения   [800]
    -H, --height <px>         высота изображения   [600]
        --samples <n>         лучей на пиксель (сглаживание) [4]
        --depth <n>           глубина отражений    [4]
    -t, --threads <n>         число потоков        [все ядра]
        --cam-pos <x,y,z>     переопределить позицию камеры
        --cam-target <x,y,z>  переопределить точку, куда смотрит камера
        --fov <градусы>       переопределить вертикальный угол обзора
    -o, --out <файл>          писать в файл вместо stdout
        --list                показать список сцен
    -h, --help                эта справка
";

struct Args {
    scene: String,
    width: usize,
    height: usize,
    samples: usize,
    depth: u32,
    threads: usize,
    cam_pos: Option<Vec3>,
    cam_target: Option<Vec3>,
    fov: Option<f64>,
    out: Option<String>,
}

fn parse_vec3(s: &str) -> Result<Vec3, String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 3 {
        return Err(format!("ожидается формат x,y,z, получено '{s}'"));
    }
    let num = |p: &str| p.trim().parse::<f64>().map_err(|_| format!("не число: '{p}'"));
    Ok(Vec3::new(num(parts[0])?, num(parts[1])?, num(parts[2])?))
}

fn parse_args() -> Result<Args, String> {
    let mut a = Args {
        scene: "3".to_string(),
        width: 800,
        height: 600,
        samples: 4,
        depth: 4,
        threads: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
        cam_pos: None,
        cam_target: None,
        fov: None,
        out: None,
    };

    let mut it = std::env::args().skip(1);
    while let Some(flag) = it.next() {
        // Для флагов со значением достаём следующий аргумент.
        let mut value = |name: &str| it.next().ok_or_else(|| format!("для {name} нужно значение"));
        match flag.as_str() {
            "-h" | "--help" => {
                print!("{}", HELP);
                process::exit(0);
            }
            "--list" => {
                for (name, num, desc) in scenes::SCENE_LIST {
                    println!("{num}  {name:<11} {desc}");
                }
                process::exit(0);
            }
            "-s" | "--scene" => a.scene = value(&flag)?,
            "-w" | "--width" => a.width = value(&flag)?.parse().map_err(|_| "некорректная ширина")?,
            "-H" | "--height" => a.height = value(&flag)?.parse().map_err(|_| "некорректная высота")?,
            "--samples" => a.samples = value(&flag)?.parse().map_err(|_| "некорректное --samples")?,
            "--depth" => a.depth = value(&flag)?.parse().map_err(|_| "некорректное --depth")?,
            "-t" | "--threads" => a.threads = value(&flag)?.parse().map_err(|_| "некорректное --threads")?,
            "--cam-pos" => a.cam_pos = Some(parse_vec3(&value(&flag)?)?),
            "--cam-target" => a.cam_target = Some(parse_vec3(&value(&flag)?)?),
            "--fov" => a.fov = Some(value(&flag)?.parse().map_err(|_| "некорректный --fov")?),
            "-o" | "--out" => a.out = Some(value(&flag)?),
            other => return Err(format!("неизвестный параметр '{other}' (см. --help)")),
        }
    }

    if a.width == 0 || a.height == 0 {
        return Err("ширина и высота должны быть > 0".into());
    }
    Ok(a)
}

fn run() -> Result<(), String> {
    let args = parse_args()?;

    let (scene, mut view) = scenes::build(&args.scene)
        .ok_or_else(|| format!("неизвестная сцена '{}' (см. --list)", args.scene))?;

    // Параметры камеры из командной строки перекрывают значения сцены.
    if let Some(p) = args.cam_pos {
        view.from = p;
    }
    if let Some(t) = args.cam_target {
        view.at = t;
    }
    if let Some(f) = args.fov {
        view.fov = f;
    }

    let aspect = args.width as f64 / args.height as f64;
    let camera = Camera::new(view.from, view.at, Vec3::UP, view.fov, aspect);

    let settings = RenderSettings {
        width: args.width,
        height: args.height,
        samples: args.samples,
        max_depth: args.depth,
        threads: args.threads,
    };

    eprintln!(
        "Сцена '{}', {}x{}, {} л/пикс., {} потоков",
        args.scene, settings.width, settings.height, settings.samples, settings.threads
    );
    let start = Instant::now();
    let pixels = renderer::render(&scene, &camera, &settings);
    eprintln!("Готово за {:.2} с", start.elapsed().as_secs_f64());

    // Вывод: файл или stdout (с буферизацией — файл может быть десятки МБ).
    let writer: Box<dyn Write> = match &args.out {
        Some(path) => Box::new(BufWriter::new(
            File::create(path).map_err(|e| format!("не удалось создать '{path}': {e}"))?,
        )),
        None => Box::new(BufWriter::new(io::stdout().lock())),
    };
    image::write_ppm(writer, settings.width, settings.height, &pixels)
        .map_err(|e| format!("ошибка записи: {e}"))
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Ошибка: {e}");
        process::exit(1);
    }
}
