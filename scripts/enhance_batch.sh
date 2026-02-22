#!/bin/bash
# enhance_batch.sh - Улучшение всех изображений

echo "🖼️  Улучшение изображений scans/ → scans_enhanced/"
mkdir -p scans_enhanced

find scans -type f \( -iname "*.jpg" -o -iname "*.jpeg" -o -iname "*.png" \) | while read -r file; do
    basename_file=$(basename "$file")
    name="${basename_file%.*}"
    output="scans_enhanced/${name}_enhanced.png"
    
    echo "✨ $basename_file → $output"
    convert "$file" \
        -resize 2000x \
        -brightness-contrast 30x40 \
        -sharpen 0x1.0 \
        -monochrome \
        "$output"
done

echo "✅ Улучшено! scans_enhanced/*.png"
ls -la scans_enhanced/
