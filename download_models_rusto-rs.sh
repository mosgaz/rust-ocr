#!/bin/bash
mkdir -p models

echo "📥 Скачиваем модели PaddleOCR v4..."
cd models

# Детекция (PP-OCRv4)
wget "https://paddleocr.bj.bcebos.com/dygraph_v2.0/multilingual/det/ch/ch_ppocr_mobile_v2.0_det_infer.tar"
tar -xf ch_ppocr_mobile_v2.0_det_infer.tar && mv ch_ppocr_mobile_v2.0_det_infer/infer_model.onnx det.onnx

# Распознавание (PP-OCRv4)
wget "https://paddleocr.bj.bcebos.com/dygraph_v2.0/multilingual/rec/ch/ch_ppocr_mobile_v2.0_rec_infer.tar"  
tar -xf ch_ppocr_mobile_v2.0_rec_infer.tar && mv ch_ppocr_mobile_v2.0_rec_infer/infer_model.onnx rec.onnx

# Русский словарь
curl -o ru_dict.txt https://raw.githubusercontent.com/PaddlePaddle/PaddleOCR/release/2.7/ppocr/utils/dict/ru_dict.txt

echo "✅ Модели готовы в models/"
ls -lh models/
