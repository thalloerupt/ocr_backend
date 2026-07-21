use image::{DynamicImage, GenericImageView, Rgb, RgbImage};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use ocr_backend::{Format, Paragraph};
use ocr_rs::{Backend, OcrEngine, OcrEngineConfig, OcrResult_};
use pdfium_render::prelude::*;
use std::path::Path;
use std::time::Instant;

const DET_MODEL_PATH: &str = "res/models/PP-OCRv6_tiny_det.mnn";
const REC_MODEL_PATH: &str = "res/models/PP-OCRv6_tiny_rec.mnn";
const DICT_PATH: &str = "res/dict/ppocr_keys_v6_tiny.txt";
const PDF_PATH: &str = "res/pdf/example.pdf";

fn main() {
    let image = export_pdf_to_image(PDF_PATH, None, 3).expect("获取图像失败");

    ocr(&image, DET_MODEL_PATH, REC_MODEL_PATH, DICT_PATH).expect("解析失败");
}

/// Renders each page in the PDF file at the given path to a separate JPEG file.
fn export_pdf_to_image(
    path: &str,
    password: Option<&str>,
    page_index: i32,
) -> Result<DynamicImage, PdfiumError> {
    // Bind to a Pdfium library in the same directory as our Rust executable.
    // See the "Dynamic linking" section below.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(Path::new(
            "./lib",
        )))
        .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    let document = pdfium.load_pdf_from_file(path, password)?;

    // ... set rendering options that will be applied to all pages...

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true);

    let page = document.pages().get(page_index).expect("获取页面失败");

    page.render_with_config(&render_config)?.as_image()
}

fn ocr(
    image: &DynamicImage,
    det_model: &str,
    rec_model: &str,
    keys_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = "debug_ocr_result.png";

    let config = OcrEngineConfig::fast().with_min_result_confidence(0.7);
    let engine = OcrEngine::new(det_model, rec_model, keys_path, Some(config))?;
    println!("   ✅ 模型加载成功");

    let (width, height) = image.dimensions();
    println!("   尺寸: {}x{}\n", width, height);

    // 3. 执行 OCR 识别
    println!("🔍 执行 OCR 识别...");
    let results = engine.recognize(&image)?;
    println!("   ✅ 检测到 {} 个文本区域\n", results.len());

    // 4. 输出详细识别结果到命令行
    //println!("╔════════════════════════════════════════════════════════════════════════╗");
    println!("                        识别结果详情                                    ");
    //println!("╠════════════════════════════════════════════════════════════════════════╣");

    // for (i, result) in results.iter().enumerate() {
    //     let bbox = &result.bbox;
    //     println!("📝 [{:2}] 文本: {}", i + 1, result.text);
    //     println!(
    //         "   置信度: {:.2}% | 位置: ({}, {}) | 尺寸: {}x{}",
    //         result.confidence * 100.0,
    //         bbox.rect.left(),
    //         bbox.rect.top(),
    //         bbox.rect.width(),
    //         bbox.rect.height()
    //     );

    //     if let Some(points) = &bbox.points {
    //         println!(
    //             "   角点: [{:.0},{:.0}] [{:.0},{:.0}] [{:.0},{:.0}] [{:.0},{:.0}]",
    //             points[0].x,
    //             points[0].y,
    //             points[1].x,
    //             points[1].y,
    //             points[2].x,
    //             points[2].y,
    //             points[3].x,
    //             points[3].y
    //         );
    //     }
    //     println!();
    // }

    //println!("╚════════════════════════════════════════════════════════════════════════╝\n");

    // 5. 可视化：绘制边界框到图像
    //println!("🎨 生成可视化结果...");
    let mut output_image = image.to_rgb8();

    // 预定义颜色方案（8种明亮的颜色）
    let colors = [
        Rgb([255u8, 0, 0]), // 红色
        Rgb([0, 255, 0]),   // 绿色
        Rgb([0, 0, 255]),   // 蓝色
        Rgb([255, 255, 0]), // 黄色
        Rgb([255, 0, 255]), // 品红
        Rgb([0, 255, 255]), // 青色
        Rgb([255, 128, 0]), // 橙色
        Rgb([128, 0, 255]), // 紫色
    ];
    let start = Instant::now();

    let paragraphs = process_page(&results);
    let duration = start.elapsed();
    println!("耗时: {} 毫秒", duration.as_millis());
    for (i, result) in paragraphs.iter().enumerate() {
        let color = colors[i % colors.len()];
        let bbox = &result.bbox;
        println!("{}", result.text);
        println!("");

        // 绘制矩形边框（绘制2次让边框更明显）
        let rect = Rect::at(bbox.left, bbox.top).of_size(bbox.width, bbox.height);

        draw_hollow_rect_mut(&mut output_image, rect, color);

        // 绘制加粗边框
        if bbox.left > 0 && bbox.top > 0 {
            let rect2 =
                Rect::at(bbox.left - 1, bbox.top - 1).of_size(bbox.width + 2, bbox.height + 2);
            draw_hollow_rect_mut(&mut output_image, rect2, color);
        }

        // 可选：绘制索引标签（如果需要在图像上显示序号）
        draw_index_label(&mut output_image, i + 1, bbox.left, bbox.top, color);
    }

    // 6. 保存可视化结果
    output_image.save(output_path)?;
    println!("   ✅ 可视化结果已保存到: {}\n", output_path);

    // 7. 统计信息
    println!("📊 统计信息:");
    if !results.is_empty() {
        let avg_confidence =
            results.iter().map(|r| r.confidence).sum::<f32>() / results.len() as f32;
        let max_confidence = results
            .iter()
            .map(|r| r.confidence)
            .fold(0.0f32, |a, b| a.max(b));
        let min_confidence = results
            .iter()
            .map(|r| r.confidence)
            .fold(1.0f32, |a, b| a.min(b));

        println!("   总文本区域数: {}", results.len());
        println!("   平均置信度:   {:.2}%", avg_confidence * 100.0);
        println!("   最高置信度:   {:.2}%", max_confidence * 100.0);
        println!("   最低置信度:   {:.2}%", min_confidence * 100.0);
    } else {
        println!("   未检测到任何文本");
    }

    println!("\n✨ 调试完成！");
    Ok(())
}

/// 在图像上绘制索引标签
fn draw_index_label(image: &mut RgbImage, _index: usize, x: i32, y: i32, color: Rgb<u8>) {
    // 计算标签位置（稍微偏移到框的左上角外侧）
    let label_x = (x - 20).max(0);
    let label_y = (y - 20).max(0);

    // 绘制标签背景（小方块）
    let label_size = 18;
    for dy in 0..label_size {
        for dx in 0..label_size {
            let px = label_x + dx;
            let py = label_y + dy;
            if px >= 0 && py >= 0 && (px as u32) < image.width() && (py as u32) < image.height() {
                image.put_pixel(px as u32, py as u32, color);
            }
        }
    }
}

fn process_page(ocr_results: &Vec<OcrResult_>) -> Vec<Paragraph> {
    let mut format = Format {
        results: ocr_results.clone(),
    };

    let mut lines = format.to_lines();

    let paragraphs = format.to_paragraphs(&mut lines);
    paragraphs
}
