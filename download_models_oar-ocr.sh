#!/bin/bash
mkdir -p models

echo "📥 Скачиваем модели OAR OCR..."
cd models

# Готовые модели для oar-ocr (меньше размер)
wget -O models/text_detection.onnx \
  https://github.com/GreatV/oar-ocr/releases/download/v0.2.0/text_detection.onnx

wget -O models/text_recognition.onnx \
  https://github.com/GreatV/oar-ocr/releases/download/v0.2.0/text_recognition.onnx

echo "✅ Модели готовы в models/"
ls -lh models/
