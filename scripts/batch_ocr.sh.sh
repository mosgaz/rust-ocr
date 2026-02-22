#!/bin/bash
# batch_ocr.sh - Массовая обработка scans/ → results/

set -e  # Остановка при ошибке

echo "🚀 rust-ocr: Массовая обработка"
echo "📁 Вход: scans/*.{jpg,jpeg,png}"
echo "📤 Выход: results/*.txt"

mkdir -p results

# Обработка всех JPG/JPEG/PNG файлов
find scans -type f \( -iname "*.jpg" -o -iname "*.jpeg" -o -iname "*.png" \) | while read -r file; do
    basename_file=$(basename "$file")
    name="${basename_file%.*}"  # Убираем расширение
    output="results/${name}.txt"
    
    echo "🔄 $basename_file → $output"
    ./target/release/ocr-scan "$file" -o "$output"
done

echo "✅ Готово! results/*.txt"
echo "📊 Файлов обработано: $(ls results/*.txt 2>/dev/null | wc -l || echo 0)"
ls -la results/
