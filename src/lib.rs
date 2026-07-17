pub mod filter;
pub mod format;
pub mod ocr;

pub use filter::Filter;
pub use format::{Format, Line, Paragraph, Rect};
pub use ocr::OcrBackend;

// Re-export ocr_rs types so users can configure the engine
pub use ocr_rs::{Backend, OcrEngineConfig};
