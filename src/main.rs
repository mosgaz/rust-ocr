mod preprocess;
mod ocr;
mod export;
mod cli;

use clap::Parser;
use std::path::{PathBuf};

#[derive(Parser)]
#[command(name = "ocr-scan")]
struct Args {
    input: PathBuf,
    #[arg(short, long, default_value = "output.md")]
    output: PathBuf,
    #[arg(long)]
    aggressive: bool,
    #[arg(long, value_enum, default_value = "oar")]
    model: ocr::OcrModel,  // oar / rusto
    #[arg(long, value_parser = parse_format)]
    format: Option<String>,
}

fn parse_format(s: &str) -> Result<String, String> {
    if ["md", "docx", "tex"].contains(&s) { Ok(s.to_string()) }
    else { Err("Формат: md, docx, tex".to_string()) }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("🚀 Модель OCR: {:?}", args.model);
    
    let config = preprocess::PreprocessConfig {
        aggressive_denoise: args.aggressive,
    };
    
    let img = preprocess::preprocess_image(&args.input, config)?;
    let texts = ocr::recognize_text_simple(&img, args.model)?;
    
    let format = args.format.unwrap_or_else(|| "md".to_string());
    let output_path = if args.output.extension().is_none() {
        let mut path = args.output.clone();
        path.set_extension(&format);
        path
    } else {
        args.output.clone()
    };
    
    match format.as_str() {
        "docx" => export::export_to_docx(&texts, &output_path)?,
        "tex" => export::export_to_latex(&texts, &output_path)?,
        "md" => export::export_to_markdown(&texts, &output_path)?,
        _ => panic!("Неподдерживаемый формат"),
    }
    
    println!("✅ Сохранено: {:?} ({} блоков)", output_path, texts.len());
    Ok(())
}
