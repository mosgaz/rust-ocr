use image::{ImageBuffer, Luma, imageops};
use std::path::Path;

#[derive(Clone, Copy)]
pub struct PreprocessConfig {
    pub aggressive_denoise: bool,
}

pub fn preprocess_image<P: AsRef<Path>>(
    path: P, 
    config: PreprocessConfig
) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    let mut img = image::open(path)?.to_luma8();
    
    // Удаление шума от шариковых ручек
    img = imageops::blur(&img, 1.5);
    
    if config.aggressive_denoise {
        // Двойная обработка для сильных помарок
        img = imageops::blur(&img, 2.0);
        for pixel in img.iter_mut() {
            *pixel = (*pixel / 4) * 4; // Квантование
        }
    }
    
    // Бинаризация для четкости текста
    let threshold = image::imageops::Threshold::new(140);
    img = threshold.apply(&img);
    
    Ok(img)
}
