use image::{ImageBuffer, Luma};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OcrError {
    #[error("OAR-OCR error")]
    Oar(String),
    #[error("Rusto-RS error")]
    Rusto(String),
    #[error("Model not found")]
    ModelNotFound,
}

#[derive(Clone, clap::ValueEnum)]
pub enum OcrModel {
    Oar,
    Rusto,
}

pub struct OcrResult {
    pub text: String,
    pub confidence: f32,
}

pub fn recognize_text(
    img: &ImageBuffer<Luma<u8>, Vec<u8>>,
    model_type: OcrModel,
    models_dir: PathBuf,
) -> Result<Vec<OcrResult>, OcrError> {
    let img_bytes: Vec<u8> = img.as_raw().to_vec();
    
    match model_type {
        OcrModel::Oar => {
            // OAR-OCR [web:13]
            let det_path = models_dir.join("text_detection.onnx");
            let rec_path = models_dir.join("text_recognition.onnx");
            
            let ocr = oar_ocr::oarocr::OAROCRBuilder::new(
                det_path.to_str().unwrap(),
                rec_path.to_str().unwrap(),
            ).map_err(|e| OcrError::Oar(e.to_string()))?;
            
            let results = ocr.recognize(&img_bytes)
                .map_err(|e| OcrError::Oar(e.to_string()))?;
                
            Ok(results.into_iter()
                .filter(|r| r.score > 0.8)
                .map(|r| OcrResult { text: r.text, confidence: r.score })
                .collect())
        }
        
        OcrModel::Rusto => {
            // Rusto-RS [web:5]
            let det_path = models_dir.join("det.onnx");
            let rec_path = models_dir.join("ch_PP-OCRv4_rec_infer.onnx");
            let dict_path = models_dir.join("ru_dict.txt");
            
            let mut engine = rusto_rs::OcrEngine::new(
                &det_path, &rec_path, &dict_path,
            ).map_err(|e| OcrError::Rusto(e.to_string()))?;
            
            let results = engine.detect_and_recognize(&img_bytes)
                .map_err(|e| OcrError::Rusto(e.to_string()))?;
                
            Ok(results.into_iter()
                .filter(|r| r.confidence > 0.85)
                .map(|r| OcrResult { text: r.text, confidence: r.confidence })
                .collect())
        }
    }
}

// Простой API
pub fn recognize_text_simple(
    img: &ImageBuffer<Luma<u8>, Vec<u8>>,
    model_type: OcrModel,
) -> Result<Vec<String>, OcrError> {
    let results = recognize_text(img, model_type, PathBuf::from("models"))?;
    Ok(results.into_iter().map(|r| r.text).collect())
}
