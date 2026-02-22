# OCR Scanner для учебных пособий

**Оптическое распознавание печатных символов (OCR)**. Автоматизирует распознавание текста из отсканированных страниц учебных пособий с помарками от шариковых ручек. Поддерживает **oar-ocr** и **rusto-rs**, экспорт в **MD/DOCX/LaTeX**.

## 📁 Структура проекта

```
rust-ocr/
├── Cargo.toml                 # Зависимости
├── README.md                  # Эти инструкции
├── scripts/
│   └── download_models.sh     # 🎯 Объединенный скрипт моделей
├── models/                    # ONNX модели (6 файлов)
│   ├── text_detection.onnx         # oar-ocr
│   ├── text_recognition.onnx       # oar-ocr  
│   ├── det.onnx                    # rusto-rs
│   ├── ch_PP-OCRv4_rec_infer.onnx  # rusto-rs
│   ├── ru_dict.txt                 # rusto-rs
│   └── ch_PP-OCRv4_det_infer.onnx  # rusto-rs (ваш файл)
├── scans/                     # ← Сюда сканы JPG/PNG/PDF
├── results/                   # ← Выходные MD/DOCX файлы
├── tests/                     # Тесты
│   └── integration.rs         # E2E тесты
└── src/                       # Исходный код
    ├── main.rs                # 🚀 Точка входа, CLI
    ├── lib.rs                 # 📦 Публичный API
    ├── cli.rs                 # 🎛️ Парсер аргументов
    ├── preprocess.rs          # 🧹 Предобработка изображений
    ├── ocr.rs                 # 🔍 OCR (oar-ocr + rusto-rs)
    └── export.rs              # 📄 Экспорт MD/DOCX/LaTeX
```

## 🚀 Быстрый старт (5 минут)

```bash
# 1. Клонировать/создать проект
git clone <repo> ocr-scanner && cd ocr-scanner

# 2. Скачать недостающие модели одним скриптом
chmod +x scripts/download_models.sh
./scripts/download_models.sh

# 3. Скомпилировать
cargo build --release

# 4. Создать папки и добавить сканы
mkdir -p scans results
cp ~/Desktop/*.jpg scans/  # Ваши сканы сюда
```

## 🎯 Использование

### Базовые команды

```bash
# Одна страница → Markdown (по умолчанию)
./target/release/ocr-scan scans/page1.jpg -o results/page1.md

# DOCX для Word/LibreOffice
./target/release/ocr-scan scans/page1.jpg -o results/page1.docx --format docx

# LaTeX для печати
./target/release/ocr-scan scans/page1.jpg -o results/page1.tex --format tex
```

### Выбор модели OCR

```bash
# 🏎️ OAR-OCR (быстрее, проще)
./target/release/ocr-scan scans/page1.jpg --model oar -o results/page1.docx --format docx

# 🎯 RUSTO-RS (лучше русский текст, рекомендую)
./target/release/ocr-scan scans/page1.jpg --model rusto -o results/page1.docx --format docx

# Максимальное качество (rusto + очистка помарок)
./target/release/ocr-scan scans/page1.jpg --model rusto --aggressive -o results/page1.docx --format docx
```

### Полные опции

```bash
./target/release/ocr-scan --help
```

```
USAGE: ocr-scan <INPUT> [OPTIONS]

ARGS:
  <INPUT>    Путь к скану JPG/PNG или папке

OPTIONS:
    -o, --output <OUTPUT>      Файл вывода [default: output.md]
    --model <MODEL>            oar / rusto [default: oar]
    --format <FORMAT>          md/docx/tex
    --aggressive               Удаление сильных помарок ручки
```

## 📊 Примеры запуска

```bash
# Быстрое тестирование
./target/release/ocr-scan scans/test.jpg --model rusto --aggressive

# Профессиональный DOCX для печати
./target/release/ocr-scan scans/page1.jpg \
  --model rusto \
  --aggressive \
  -o results/учебник.docx \
  --format docx

# Markdown для GitHub/просмотра
./target/release/ocr-scan scans/page1.jpg -o results/page1.md
```

## 🧪 Результат

**results/page1.docx** содержит:
```
# Распознанный тест из учебника

## Вопрос 1:
1. Что такое Rust?
A) Язык программирования ✓
B) Фрукт ✗
C) Автомобиль ✗

## Вопрос 2:
2. Преимущества Rust:
- Безопасность памяти
- Производительность
- Современный синтаксис
```

## 🔧 Устранение неисправностей

| Проблема | Решение |
|----------|---------|
| `Model not found` | `./scripts/download_models.sh` |
| `No such file` | `mkdir -p scans results` |
| Медленно | `--model oar` |
| Плохие помарки | `--aggressive` |
| Плохое качество | `--model rusto` |

## 📈 Сравнение моделей

| Модель | Скорость | Русский текст | Помарки ручки |
|--------|----------|---------------|---------------|
| **oar** | 🏎️ 2-3 сек | Хорошо | ⭐⭐⭐⭐ |
| **rusto** | 🐌 4-6 сек | **⭐⭐⭐⭐⭐** | ⭐⭐⭐ |

**Рекомендация**: `--model rusto --aggressive` для учебников с помарками.

## 📝 Лицензия

MIT License. Используйте для любых образовательных целей!