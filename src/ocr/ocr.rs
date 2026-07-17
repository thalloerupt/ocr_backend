use image::DynamicImage;
use ocr_rs::{DetOptions, OcrEngine, OcrEngineConfig, OcrResult_};

use crate::format::{Format, Paragraph};

/// OCR 引擎配置
///
/// # 示例
/// ```no_run
/// use ocr_backend::OcrBackendConfig;
///
/// let config = OcrBackendConfig::new()
///     .with_threads(4)
///     .with_min_result_confidence(0.7);
/// ```
pub struct OcrBackendConfig {
    threads: i32,
    min_result_confidence: f32,
    fast_det: bool,
}

impl Default for OcrBackendConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl OcrBackendConfig {
    pub fn new() -> Self {
        Self {
            threads: 4,
            min_result_confidence: 0.7,
            fast_det: false,
        }
    }

    pub fn with_threads(mut self, threads: i32) -> Self {
        self.threads = threads;
        self
    }

    pub fn with_min_result_confidence(mut self, confidence: f32) -> Self {
        self.min_result_confidence = confidence;
        self
    }

    pub fn with_fast_det(mut self, fast: bool) -> Self {
        self.fast_det = fast;
        self
    }

    fn into_inner(self) -> OcrEngineConfig {
        let config = OcrEngineConfig::new()
            .with_backend(ocr_rs::Backend::CPU)
            .with_threads(self.threads)
            .with_min_result_confidence(self.min_result_confidence);
        if self.fast_det {
            config.with_det_options(DetOptions::fast())
        } else {
            config
        }
    }
}

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
    /// use ocr_backend::{OcrBackend, OcrBackendConfig};
    ///
    /// let config = OcrBackendConfig::new().with_min_result_confidence(0.7);
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
        config: Option<OcrBackendConfig>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let engine_config = config.map(OcrBackendConfig::into_inner);
        let engine = OcrEngine::new(det_model, rec_model, dict_path, engine_config)?;
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
