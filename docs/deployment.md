# 🚀 Развертывание rust-ocr v0.2.0 (Ubuntu/Debian)

## 1. Системные зависимости

```bash
# Обновление системы
sudo apt update && sudo apt upgrade -y

# Tesseract OCR (русский + английский)
sudo apt install -y tesseract-ocr tesseract-ocr-rus tesseract-ocr-eng

# Обработка изображений
sudo apt install -y imagemagick libmagickwand-dev pkg-config

# Rust toolchain (если не установлен)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup default stable
```

## 2. Клонирование и сборка

```bash
# Создать проектную директорию
mkdir -p ~/projects/rust/ocr
cd ~/projects/rust/ocr

# Клонировать rust-ocr (или скопировать ваш проект)
git clone <your-repo> rust-ocr  # или cp -r rust-ocr rust-ocr-deploy
cd rust-ocr

# Сборка продакшн версии
cargo build --release
```

## 3. Права на исполнение

```bash
chmod +x scripts/*.sh
chmod +x target/release/ocr-scan
sudo ln -s $(pwd)/target/release/ocr-scan /usr/local/bin/ocr-scan
```

## 4. Настройка конфигурации

```bash
# Создать .env (если отсутствует)
cat > .env << 'EOF'
OCR_TESSERACT_LANG=rus
OCR_TESSERACT_PSM=6
OCR_TESSERACT_DPI=300
ENHANCE_RESIZE_WIDTH=1800
ENHANCE_BRIGHTNESS=20
ENHANCE_CONTRAST=25
ENHANCE_SHARPEN_RADIUS=0.8
EOF
```

## 5. Создать рабочие директории

```bash
mkdir -p scans results temp
cp docs/учебник/*.jpg scans/ 2>/dev/null || true
```

## 6. Тестирование развертывания

```bash
# ✅ Проверка сборки
cargo test          # 3/3 GREEN

# ✅ Проверка бинарника
./target/release/ocr-scan --version

# ✅ Тестовый запуск
./scripts/single_ocr.sh tests/fixtures/page1.jpg

# ✅ Проверка результата
grep "АВТОМАТИЗАЦИЯ" results/page1.txt && echo "✅ OCR РАБОТАЕТ!"
```

## 7. Продакшн пайплайн

```bash
# Массовое размножение учебников
cp ~/Desktop/учебник/*.jpg scans/
./scripts/batch_ocr.sh
cat results/*.txt > учебник_полный.txt
wc -l учебник_полный.txt  # Количество строк
```

## 8. Мониторинг и логи

```bash
# Логи обработки
tail -f results/*.log 2>/dev/null || echo "Логи в results/*.txt"

# Статистика проекта
find results -name "*.txt" | xargs wc -l
```

## 9. Автозапуск (systemd опционально)

```bash
# Создать сервис для cron
echo "0 9 * * 1-5 cd /home/\$USER/projects/rust/ocr/rust-ocr && ./scripts/batch_ocr.sh" | crontab -
```

## ✅ Критерии успешного развертывания

```
[ ] cargo build --release = 0 warnings
[ ] cargo test = 3/3 GREEN тестов
[ ] ./scripts/batch_ocr.sh = results/*.txt
[ ] grep "АВТОМАТИЗАЦИЯ" results/page1.txt
[ ] 21+ блоков текста на страницу
[ ] Время обработки ≤30 сек/страница
```

## 📊 Системные требования

| Компонент | Минимальная версия |
|-----------|-------------------|
| **Ubuntu** | 20.04 LTS |
| **Rust** | 1.75+ |
| **Tesseract** | 4.1.1+ |
| **RAM** | 4GB |
| **Диск** | 2GB |

---

**Дата развертывания**: $(date)
**Статус**: ✅ **ПРОДАКШЕН ГОТОВ**
```

### **🚀 Быстрая команда развертывания:**

```bash
cd ~/projects/rust/ocr/rust-ocr
mkdir -p docs && cat > docs/deployment.md << 'EOF'
[вставить содержимое выше]
EOF
echo "✅ docs/deployment.md СОЗДАН!"
```

## ✅ **Теперь структура полная:**

```
docs/
├── ТЗ.md          # Уже есть
└── deployment.md  # ✅ НОВЫЙ!
```