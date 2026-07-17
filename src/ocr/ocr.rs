use image::DynamicImage;
use ocr_rs::{OcrEngine, OcrEngineConfig, OcrResult_};

use crate::format::{Format, Paragraph};

pub struct OcrBackend {
    engine: OcrEngine,
}

impl OcrBackend {
    /// 创建 OCR 引擎并加载模型
    ///
    /// # 参数
    /// * `det_model` - 检测模型路径
    /// * `rec_model` - 识别模型路径
    /// * `dict_path` - 字典文件路径
    /// * `config` - 可选的引擎配置
    ///
    /// # 示例
    /// ```no_run
    /// use ocr_backend::OcrBackend;
    /// use ocr_rs::OcrEngineConfig;
    ///
    /// let config = OcrEngineConfig::fast().with_min_result_confidence(0.7);
    /// let backend = OcrBackend::new(
    ///     "res/models/PP-OCRv6_tiny_det.mnn",
    ///     "res/models/PP-OCRv6_tiny_rec.mnn",
    ///     "res/dict/ppocr_keys_v6_tiny.txt",
    ///     Some(config),
    /// ).unwrap();
    /// ```
    pub fn new(
        det_model: &str,
        rec_model: &str,
        dict_path: &str,
        config: Option<OcrEngineConfig>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let engine = OcrEngine::new(det_model, rec_model, dict_path, config)?;
        Ok(Self { engine })
    }

    /// 识别图像中的文本，返回格式化后的段落列表
    pub fn recognize(
        &self,
        image: &DynamicImage,
    ) -> Result<Vec<Paragraph>, Box<dyn std::error::Error>> {
        let results = self.engine.recognize(image)?;
        let mut format = Format { results };
        let lines = format.to_lines();
        let paragraphs = format.to_paragraphs(&lines);
        Ok(paragraphs)
    }

    /// 识别图像中的文本，返回原始 OCR 结果
    pub fn recognize_raw(
        &self,
        image: &DynamicImage,
    ) -> Result<Vec<OcrResult_>, Box<dyn std::error::Error>> {
        let results = self.engine.recognize(image)?;
        Ok(results)
    }
}
