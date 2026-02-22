#!/bin/bash
mkdir -p models

echo "📥 Скачиваем OAR-OCR модели..."
curl -L -o models/text_detection.onnx \
  https://github.com/GreatV/oar-ocr/releases/download/v0.2.0/text_detection.onnx
curl -L -o models/text_recognition.onnx \
  https://github.com/GreatV/oar-ocr/releases/download/v0.2.0/text_recognition.onnx

echo "📥 Скачиваем Rusto-RS модели..."
curl -L -o models/det.onnx https://paddleocr.ppocr.ru/det.onnx
curl -L -o models/ch_PP-OCRv4_rec_infer.onnx https://paddleocr.ppocr.ru/ch_PP-OCRv4_rec_infer.onnx
curl -L -o models/ru_dict.txt https://paddleocr.ppocr.ru/ru_dict.txt

echo "✅ Все модели готовы в models/"
ls -la models/
