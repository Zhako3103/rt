//! Запись изображения в текстовый формат PPM (P3).

use std::io::{self, Write};

/// Формат P3:
/// ```text
/// P3
/// <ширина> <высота>
/// 255
/// R G B   (по одному пикселю на строку, слева направо, сверху вниз)
/// ```
/// `pixels` — плоский буфер RGB, длина = ширина * высота * 3.
pub fn write_ppm<W: Write>(mut out: W, width: usize, height: usize, pixels: &[u8]) -> io::Result<()> {
    debug_assert_eq!(pixels.len(), width * height * 3);
    writeln!(out, "P3\n{} {}\n255", width, height)?;
    for px in pixels.chunks_exact(3) {
        writeln!(out, "{} {} {}", px[0], px[1], px[2])?;
    }
    out.flush()
}
