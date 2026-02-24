// tests/integration.rs - Полный пайплайн OCR
use rust_ocr::{export_to_markdown, load_config, preprocess_image, recognize_text_simple};
use std::path::Path;

#[test]
fn test_full_ocr_pipeline() {
    // 1. Загрузить конфигурацию
    let config = load_config();

    // 2. Тестовый файл (скопируйте scans/page1.jpg → tests/fixtures/)
    let input_path = "tests/fixtures/page1.jpg";

    // 3. Полный пайплайн: preprocess → OCR → export
    let img = preprocess_image(input_path, &config).expect("Предобработка провалилась");
    let texts = recognize_text_simple(&img, &config.ocr).expect("OCR провалился");

    // 4. Проверки качества
    assert!(img.width() > 1000, "Размер должен быть увеличен");
    assert!(texts.len() >= 15, "Минимум 15 блоков текста");
    assert!(texts[0].contains("АВТОМАТИЗАЦИЯ"), "Заголовок не распознан");
    assert!(
        texts.iter().any(|t| t.contains("классификации")),
        "Основной текст не найден"
    );

    println!(
        "✅ Integration test: {}x{}, {} блоков текста",
        img.width(),
        img.height(),
        texts.len()
    );

    // 5. Экспорт (тестовый файл)
    let temp_output = "tests/temp_test_output.txt";
    export_to_markdown(&texts, Path::new(temp_output)).expect("Экспорт провалился");

    // 6. Проверка файла
    let exported = std::fs::read_to_string(temp_output).expect("Файл не создан");
    assert!(!exported.is_empty(), "Экспорт пустой");

    // Очистка
    std::fs::remove_file(temp_output).ok();
}

#[test]
fn test_preprocess_quality() {
    let config = load_config();
    let img = preprocess_image("scans/page1.jpg", &config).expect("Предобработка");

    assert!(img.width() >= 1400, "Ширина должна быть увеличена");
    assert!(img.height() > 800, "Высота должна быть пропорциональна");
    println!("✅ Preprocess: {}x{}", img.width(), img.height());
}

#[test]
fn test_ocr_minimum_blocks() {
    let config = load_config();
    let img = preprocess_image("scans/page1.jpg", &config).expect("Предобработка");
    let texts = recognize_text_simple(&img, &config.ocr).expect("OCR");

    assert!(texts.len() >= 19, "Должно быть минимум 19 блоков");
    assert!(
        texts.first().unwrap().contains("АВТОМАТИЗАЦИЯ"),
        "Нет заголовка"
    );
    println!(
        "✅ OCR: {} блоков, первый: {}",
        texts.len(),
        &texts[0][..50]
    );
}
