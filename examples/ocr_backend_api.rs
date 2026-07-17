//! 演示 OcrBackend 的使用：先创建引擎加载模型，再传入 image 识别
//!
//! 用法: cargo run --example ocr_backend_api

use ocr_backend::{OcrBackend, OcrBackendConfig};

const DET_MODEL_PATH: &str = "res/models/PP-OCRv6_tiny_det.mnn";
const REC_MODEL_PATH: &str = "res/models/PP-OCRv6_tiny_rec.mnn";
const DICT_PATH: &str = "res/dict/ppocr_keys_v6_tiny.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建引擎，加载模型（只需一次）
    let config = OcrBackendConfig::new()
        .with_threads(4)
        .with_min_result_confidence(0.7)
        .with_fast_det(true);
    let backend = OcrBackend::new(DET_MODEL_PATH, REC_MODEL_PATH, DICT_PATH, Some(config))?;
    println!("✅ 模型加载成功");

    // 2. 传入图片进行识别（可多次调用）
    // let image = image::open("path/to/image.png")?;
    // let paragraphs = backend.recognize(&image)?;
    // for para in &paragraphs {
    //     println!("{}", para.text);
    // }

    // 也可以获取原始 OCR 结果
    // let raw_results = backend.recognize_raw(&image)?;
    // for result in &raw_results {
    //     println!("{} (置信度: {:.1}%)", result.text, result.confidence * 100.0);
    // }

    Ok(())
}
