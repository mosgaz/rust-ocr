use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn export_to_markdown(texts: &[String], output: &Path) -> std::io::Result<()> {
    let mut file = File::create(output)?;
    writeln!(file, "# Распознанный тест")?;
    
    for (i, text) in texts.iter().enumerate() {
        writeln!(file, "## Вопрос {}:", i + 1)?;
        writeln!(file, "{}", text.replace("•", "-"))?;
        writeln!(file)?;
    }
    Ok(())
}

pub fn export_to_latex(texts: &[String], output: &Path) -> std::io::Result<()> {
    let mut file = File::create(output)?;
    writeln!(file, r#"\documentclass{{article}}"#)?;
    writeln!(file, r#"\usepackage[utf8]{{inputenc}}"#)?;
    writeln!(file, r#"\usepackage[russian]{{babel}}"#)?;
    writeln!(file, r#"\begin{{document}}"#)?;
    
    for (i, text) in texts.iter().enumerate() {
        writeln!(file, r#"\subsection{{Вопрос {}}}"#, i + 1)?;
        writeln!(file, "{}", text.replace("-", r"$\bullet$"))?;
    }
    
    writeln!(file, r#"\end{{document}}"#)?;
    Ok(())
}
