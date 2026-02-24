use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn export_to_markdown(texts: &[String], output: &Path) -> std::io::Result<()> {
    let mut file = File::create(output)?;

    for text in texts {
        if !text.trim().is_empty() {
            writeln!(file, "{}", text.trim())?;
            writeln!(file)?; // Пустая строка между блоками
        }
    }
    Ok(())
}
