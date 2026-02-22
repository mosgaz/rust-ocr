use image::{ImageBuffer, Luma};
use std::path::Path;

#[derive(Clone, Copy)]
pub struct PreprocessConfig {
	 #[allow(dead_code)]
    pub aggressive_denoise: bool,
}

pub fn preprocess_image<P: AsRef<Path>>(
    path: P,
    _config: PreprocessConfig
) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    println!("🧹 Минимальная предобработка");
    
    // БЕЗ blur и binary - только luma конвертация
    let img = image::open(path)?.to_luma8();
    
    println!("📐 Размер: {}x{}", img.width(), img.height());
    Ok(img)
}
