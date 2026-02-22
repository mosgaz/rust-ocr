mod preprocess;
mod ocr;
mod export;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ocr-scan")]
struct Args {
    input: PathBuf,
    #[arg(short, long, default_value = "output.md")]
    output: PathBuf,
    #[arg(long)]
    aggressive: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("🔍 OCR распознавание: {:?}", args.input);
    
    let config = preprocess::PreprocessConfig {
        aggressive_denoise: args.aggressive,
    };
    
    let img = preprocess::preprocess_image(&args.input, config)?;
    println!("🖼️  Изображение обработано: {}x{}", img.width(), img.height());
    
    let texts = ocr::recognize_text_simple(&img)?;
    println!("✅ Распознано {} блоков текста", texts.len());
    
    export::export_to_markdown(&texts, &args.output)?;
    println!("💾 Сохранено: {:?}", args.output);
    
    Ok(())
}
