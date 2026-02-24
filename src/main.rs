mod config;
mod export;
mod ocr;
mod preprocess;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ocr-scan")]
struct Args {
    input: PathBuf,
    #[arg(short, long, default_value = "output.txt")]
    output: PathBuf,
    #[arg(long)]
    aggressive: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config();
    let args = Args::parse();

    println!("🔍 OCR распознавание: {:?}", args.input);

    let img = preprocess::preprocess_image(&args.input, &config)?;
    println!(
        "🖼️  Изображение обработано: {}x{}",
        img.width(),
        img.height()
    );

    let texts = ocr::recognize_text_simple(&img, &config.ocr)?;
    println!("✅ Распознано {} блоков текста", texts.len());

    export::export_to_markdown(&texts, &args.output)?;
    println!("💾 Сохранено: {:?}", args.output);

    Ok(())
}
