use image::{ImageBuffer, Luma, DynamicImage};
use rusty_tesseract::{Image, Args};
use std::fs;

pub fn recognize_text_simple(
    img: &ImageBuffer<Luma<u8>, Vec<u8>>,
) -> Result<Vec<String>, String> {
    println!("📸 OCR: размер изображения {}x{}", img.width(), img.height());
    
    let temp_path = "temp_ocr.png";
    let dynamic_img = DynamicImage::ImageLuma8(img.clone());
    dynamic_img.save(temp_path).map_err(|e| e.to_string())?;
    
    let tesseract_img = Image::from_path(temp_path)
        .map_err(|e| format!("Image::from_path error: {}", e))?;
    
    let mut args = Args::default();
    args.lang = "rus+eng".to_string();
    args.psm = Some(3);  // Полностью автоматическое разделение (лучше для тестов)
    
    println!("🔧 Tesseract PSM=3, lang='rus+eng'");
    
    let output = rusty_tesseract::image_to_string(&tesseract_img, &args)
        .map_err(|e| format!("Tesseract FAILED: {}", e))?;
    
    println!("📝 СЫРОЙ Tesseract вывод:\n{}", output);
    
    let _ = fs::remove_file(temp_path);
    
    // 🆕 ОСЛАБЛЕННЫЙ ФИЛЬТР - пропускаем больше текста
    let mut blocks = vec![];
    for (i, line) in output.lines().enumerate() {
        let trimmed = line.trim();
        println!("Строка {}: '{}'", i, trimmed);
        
        // Пропускаем совсем пустые, но оставляем короткие слова
        if !trimmed.is_empty() && trimmed.len() > 1 {
            blocks.push(trimmed.to_string());
        }
    }
    
    // println!("🔍 После фильтра: {} строк", blocks.len());
    // Ok(blocks)

	let clean_blocks: Vec<String> = blocks
		.into_iter()
		.filter(|block| !block.trim().is_empty())  // Только непустые
		.collect();

	println!("🔍 Финальный результат: {} блоков", clean_blocks.len());
	Ok(clean_blocks)
}
