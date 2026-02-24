use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OcrConfig {
    pub tesseract_lang: String,
    pub tesseract_psm: i32, // ✅ i32 вместо u32
    #[serde(default = "default_dpi")]
    pub tesseract_dpi: i32, // ✅ i32 вместо u32
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnhanceConfig {
    pub resize_width: u32,
    pub brightness: i32,
    #[allow(dead_code)]
    pub contrast: i32,
    #[allow(dead_code)]
    pub sharpen_radius: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub ocr: OcrConfig,
    pub enhance: EnhanceConfig,
}

fn default_dpi() -> i32 {
    300
}
#[allow(dead_code)]
fn default_sharpen() -> f32 {
    1.0
}

// В impl Default for AppConfig:
impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ocr: OcrConfig {
                tesseract_lang: "rus+eng".to_string(), //
                // tesseract_lang: "rus".to_string(),		// rus вместо rus+eng (?)
                tesseract_psm: 6, // PSM=6 для колонок
                tesseract_dpi: 300,
            },
            enhance: EnhanceConfig {
                resize_width: 1800, // Увеличение
                brightness: 40,
                contrast: 50,
                sharpen_radius: 1.0,
            },
        }
    }
}

pub fn load_config() -> AppConfig {
    let config = AppConfig::default();

    println!("⚙️  Загрузка конфигурации:");
    println!(
        "   OCR: {}, PSM={}, DPI={}",
        config.ocr.tesseract_lang, config.ocr.tesseract_psm, config.ocr.tesseract_dpi
    );
    println!(
        "   Enhance: {}x, яркость+{}, контраст+{}",
        config.enhance.resize_width, config.enhance.brightness, config.enhance.contrast
    );

    config
}
