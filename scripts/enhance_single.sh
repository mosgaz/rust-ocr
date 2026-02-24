#!/bin/bash
# enhance_single.sh - Улучшение одного изображения

if [ $# -eq 0 ]; then
    echo "Использование: $0 <input.jpg> [output.png]"
    echo "Пример: $0 scans/page1.jpg scans/page1_enhanced.png"
    exit 1
fi

input="$1"
output="${2:-$(dirname "$1")/$(basename "$1" .jpg)_enhanced.png}"

echo "🖼️  Улучшение: $input → $output"
convert "$input" \
    -resize 2000x \
    -brightness-contrast 30x40 \
    -sharpen 0x1.0 \
    -monochrome \
    "$output"

echo "✅ Готово: $output"
