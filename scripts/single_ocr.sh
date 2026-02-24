#!/bin/bash
# single_ocr.sh - Распознавание одного файла

if [ $# -eq 0 ]; then
    echo "Использование: $0 <input.jpg> [output.txt]"
    echo "Пример: $0 scans/page1.jpg results/page1.txt"
    exit 1
fi

input="$1"
output="${2:-results/$(basename "$1" | sed 's/\.[^.]*$//').txt}"

# Создать папки
mkdir -p results

echo "🔍 Распознавание: $input → $output"
./target/release/ocr-scan "$input" -o "$output"
echo "✅ Готово: $output"
cat "$output" | head -10
