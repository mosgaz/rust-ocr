use image::{imageops, ImageBuffer, Luma};
use std::path::Path;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct PreprocessConfig {
    pub aggressive_denoise: bool,
}

pub fn preprocess_image<P: AsRef<Path>>(
    path: P,
    app_config: &super::config::AppConfig,
) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    println!(
        "🧹 Предобработка: {}x яркость+{}",
        app_config.enhance.resize_width, app_config.enhance.brightness
    );

    let img = image::open(path)?.to_luma8();
    println!("📐 Исходный: {}x{}", img.width(), img.height());

    // Вычисление высоты
    let height = ((img.height() as f32 * app_config.enhance.resize_width as f32
        / img.width() as f32) as u32)
        .max(1);

    // Масштабирование
    let resized = imageops::resize(
        &img,
        app_config.enhance.resize_width,
        height,
        image::imageops::FilterType::Lanczos3,
    );

    // Мягкий контраст для заголовков
    let contrasted = imageops::contrast(&resized, 0.25);

    // Мягкая яркость
    let bright = imageops::brighten(&contrasted, 20);

    println!(
        "✅ {}x{} → {}x{}",
        img.width(),
        img.height(),
        bright.width(),
        bright.height()
    );

    Ok(bright)
}
