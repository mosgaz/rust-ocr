use image::{DynamicImage, ImageBuffer, Luma};
use rusty_tesseract::{Args, Image};
use std::fs;

pub fn recognize_text_simple(
    img: &ImageBuffer<Luma<u8>, Vec<u8>>,
    config: &super::config::OcrConfig,
) -> Result<Vec<String>, String> {
    println!(
        "📸 OCR: lang={}, PSM={}",
        config.tesseract_lang, config.tesseract_psm
    );

    let temp_path = "temp_ocr.png";

    // Сохранить для Tesseract
    let dynamic_img = DynamicImage::ImageLuma8(img.clone());
    dynamic_img.save(temp_path).map_err(|e| e.to_string())?;

    let tesseract_img =
        Image::from_path(temp_path).map_err(|e| format!("Image::from_path error: {}", e))?;

    // Настройки Tesseract из config
    let mut args = Args::default();
    args.lang = config.tesseract_lang.clone();
    args.psm = Some(config.tesseract_psm);
    args.dpi = Some(config.tesseract_dpi);

    let output = rusty_tesseract::image_to_string(&tesseract_img, &args)
        .map_err(|e| format!("Tesseract error: {}", e))?;

    // Удалить временный файл
    let _ = fs::remove_file(temp_path);

    // Разбить на блоки текста
    let mut blocks = vec![];
    for (_i, line) in output.lines().enumerate() {
        let trimmed = line.trim();
        if !trimmed.is_empty() && trimmed.len() > 2 {
            blocks.push(trimmed.to_string());
        }
    }

    // ------------------------
    // ------------------------
    // ------------------------
    println!("🔍 Распознано {} блоков", blocks.len());
    Ok(blocks)
    // ------------------------
    // ------------------------
    // ------------------------
    // let clean_blocks: Vec<String> = blocks
    // 	.into_iter()
    // 	.filter(|block| !block.trim().is_empty())  // Только непустые
    // 	.collect();

    // println!("🔍 Финальный результат: {} блоков", clean_blocks.len());
    // Ok(clean_blocks)
    // ------------------------
    // ------------------------
    // ------------------------
}
