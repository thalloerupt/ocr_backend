use crate::filter::Filter;
use ocr_rs::OcrResult_;
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub top: i32,
    pub left: i32,
    pub width: u32,
    pub height: u32,
}
impl Default for Rect {
    fn default() -> Self {
        Self {
            top: 0,
            left: 0,
            width: 0,
            height: 0,
        }
    }
}
impl Rect {
    fn bottom(&self) -> i32 {
        self.top + self.height as i32
    }
    fn right(&self) -> i32 {
        self.left + self.width as i32
    }

    /// 判断当前矩形是否与另一个矩形重叠（包括边界接触）
    pub fn overlaps(&self, other: &Rect) -> bool {
        // 计算右边界和下边界
        let self_right = self.left + self.width as i32;
        let self_bottom = self.top + self.height as i32;
        let other_right = other.left + other.width as i32;
        let other_bottom = other.top + other.height as i32;

        // 判断是否重叠：在x轴和y轴上都有交集
        self.left < other_right
            && self_right > other.left
            && self.top < other_bottom
            && self_bottom > other.top
    }

    fn is_same_column(&self, b: &Rect) -> bool {
        self.left < b.right() && b.left < self.right()
    }
    fn is_same_section(&self, b: &Rect, height: i32) -> bool {
        //let height = 40;
        if self.top > b.top {
            (-height..height).contains(&(&self.bottom() - b.top))
        } else {
            (-height..height).contains(&(&self.top - b.bottom()))
        }
    }

    /// 合并两个矩形为外接矩形（能同时包含这两个矩形的最小矩形）
    fn union_rect(&self, b: &Rect) -> Rect {
        let top = self.top.min(b.top);
        let left = self.left.min(b.left);
        let bottom = self.bottom().max(b.bottom());
        let right = self.right().max(b.right());

        Rect {
            top,
            left,
            width: (right - left) as u32,
            height: (bottom - top) as u32,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Line {
    pub index: usize,
    pub bbox: Rect,
    pub text: String,
}

pub struct Paragraph {
    pub bbox: Rect,
    pub text: String,
    pub lines: Vec<usize>,
    pub font_size: u32,
}

fn median_height(lines: &[Line]) -> f64 {
    let mut heights: Vec<u32> = lines.iter().step_by(4).map(|l| l.bbox.height).collect();
    heights.sort_unstable();
    let n = heights.len();
    if n == 0 {
        return 0.0;
    }
    if n % 2 == 1 {
        heights[n / 2] as f64
    } else {
        (heights[n / 2 - 1] as f64 + heights[n / 2] as f64) / 2.0
    }
}

pub struct Format {
    pub results: Vec<OcrResult_>,
}

impl Format {
    pub fn to_lines(&mut self) -> Vec<Line> {
        let mut lines = Vec::new();
        for (i, result) in self.results.iter().enumerate() {
            let _rect = result.bbox.rect;
            let line = Line {
                bbox: Rect {
                    top: _rect.top(),
                    left: _rect.left(),
                    width: _rect.width(),
                    height: _rect.height(),
                },
                text: result.text.clone(),
                index: i,
            };
            lines.push(line);
        }
        lines
    }

    pub fn to_paragraphs(&mut self, lines: &Vec<Line>) -> Vec<Paragraph> {
        let mut paragraphs: Vec<Paragraph> = Vec::new();
        let mut _paragraphs: Vec<Paragraph> = Vec::new();

        // let mut text = "".to_string();
        // let mut bbox: Rect = Rect::default();
        let height = median_height(lines) as i32;

        let _lines = filter(lines);

        for line in &_lines {
            if let Some(para) = paragraphs.iter_mut().find(|p| {
                p.bbox.is_same_column(&line.bbox) && p.bbox.is_same_section(&line.bbox, height)
                    || p.bbox.overlaps(&line.bbox)
            }) {
                // 匹配到了，更新
                para.bbox = para.bbox.union_rect(&line.bbox);
                para.lines.push(line.index);
                continue;
            } else {
                // 没匹配到，新增段落
                paragraphs.push(Paragraph {
                    bbox: line.bbox,
                    text: "".to_string(),
                    lines: vec![line.index],
                    font_size: 7,
                });
            }
        }

        for paragraph in &mut paragraphs {
            let mut lines1: Vec<Line> = Vec::new();
            for index in &paragraph.lines {
                if let Some(line) = lines.get(*index) {
                    lines1.push(line.clone());
                } else {
                    println!("索引 {} 越界，_lines 长度: {}", index, _lines.len());
                }
            }
            lines1.sort_by_key(|line| line.bbox.top);
            let font_size = estimate_font_size_mixed(2000, &lines1[0].text, lines1[0].bbox.width);
            paragraph.text = lines1
                .iter()
                .map(|l| l.text.as_str())
                .collect::<Vec<_>>()
                .join("\n");
            paragraph.font_size = font_size;
        }

        paragraphs
    }
}

fn filter(lines: &Vec<Line>) -> Vec<Line> {
    let _filter = Filter {
        results: lines.clone(),
    };
    _filter.font_filter()
}

// /// 合并一组矩形为外接矩形
// fn union(rects: &[&Rect]) -> Rect {
//     let top = rects.iter().map(|r| r.top).min().unwrap();
//     let left = rects.iter().map(|r| r.left).min().unwrap();
//     let bottom = rects.iter().map(|r| r.bottom()).max().unwrap();
//     let right = rects.iter().map(|r| r.right()).max().unwrap();
//     Rect {
//         top,
//         left,
//         width: (right - left) as u32,
//         height: (bottom - top) as u32,
//     }
// }
//
//

/// 估算在给定页面宽度下，容纳指定文本内容所需的最大字号
///
/// # 参数
/// * `page_width` - 页面可用宽度（单位：像素，u32）
/// * `content` - 要显示的文本内容
/// * `max_lines` - 最大允许的行数（u32）
/// * `char_width_ratio` - 字符宽度与字号的比例系数
///   - 等宽字体：约 0.6-0.7
///   - 中文字体：约 0.9-1.0
///   - 西文比例字体：约 0.4-0.6
///
/// # 返回
/// 估算的合适字号（单位：像素，u32）
fn estimate_font_size(
    page_width: u32,
    content: &str,
    max_lines: u32,
    char_width_ratio: f64,
) -> u32 {
    // 去除空白字符，计算有效字符数
    let char_count = content.chars().filter(|c| !c.is_whitespace()).count() as u32;

    if char_count == 0 || max_lines == 0 {
        return 16; // 默认字号
    }

    // 每行平均字符数（向上取整）
    let chars_per_line = (char_count + max_lines - 1) / max_lines;

    // 估算字号：页面宽度 / (每行字符数 * 字符宽度系数)
    let font_size = page_width as f64 / (chars_per_line as f64 * char_width_ratio);

    // 限制在合理范围内 (8-72像素)
    (font_size.round() as u32).clamp(8, 72)
}

/// 增强版：考虑中英文混合的文本，自动计算字符宽度比例
fn estimate_font_size_mixed(page_width: u32, content: &str, max_lines: u32) -> u32 {
    let mut cjk_count: u32 = 0;
    let mut latin_count: u32 = 0;

    for ch in content.chars().filter(|c| !c.is_whitespace()) {
        // 判断是否为CJK字符
        if ('\u{4E00}'..='\u{9FFF}').contains(&ch)   // 基本汉字
            || ('\u{3400}'..='\u{4DBF}').contains(&ch)  // 扩展A
            || ('\u{FF00}'..='\u{FFEF}').contains(&ch)  // 全角字符
            || ('\u{3000}'..='\u{303F}').contains(&ch)
        // CJK标点
        {
            cjk_count += 1;
        } else {
            latin_count += 1;
        }
    }

    // CJK字符宽度约为字号的1倍，拉丁字符约为0.5倍
    let equivalent_chars = cjk_count as f64 * 1.0 + latin_count as f64 * 0.5;

    if equivalent_chars == 0.0 || max_lines == 0 {
        return 16;
    }

    let chars_per_line = equivalent_chars / max_lines as f64;
    let font_size = page_width as f64 / chars_per_line;

    (font_size.round() as u32).clamp(8, 72)
}
