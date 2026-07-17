use crate::{Paragraph, format::Line};

pub struct Filter {
    //pub results: Vec<Line>,
}

impl Filter {
    pub fn front_filter(&self, results: &Vec<Line>) -> Vec<Line> {
        let mut _lines: Vec<Line> = Vec::new();
        let width = avg_char_width(results);
        for line in results {
            if is_short(line.text.clone()) {
                continue;
            }
            // if is_single_word(line.text.clone()) {
            //     continue;
            // }
            if is_mostly_numeric(line.text.clone()) {
                continue;
            }
            if char_width(line.text.clone(), line.bbox.width) + 2 < width {
                continue;
            }
            // if contains_math_symbols(line.text.clone()) {
            //     continue;
            // }
            _lines.push(Line {
                index: line.index,
                bbox: line.bbox,
                text: line.text.clone(),
            });
        }
        _lines
    }

    pub fn back_filter(&self, paragraphs: Vec<Paragraph>) -> Vec<Paragraph> {
        let mut _paragraphs: Vec<Paragraph> = Vec::new();
        for paragraph in &paragraphs {
            if is_single_line(paragraph) && contains_math_symbols(paragraph.text.clone()) {
                break;
            }
            _paragraphs.push(Paragraph {
                bbox: paragraph.bbox,
                text: paragraph.text.clone(),
                lines: paragraph.lines.clone(),
                font_size: paragraph.font_size,
            });
        }

        _paragraphs
    }
}

pub fn is_single_line(paragraph: &Paragraph) -> bool {
    paragraph.lines.len() == 1
}

fn is_single_word(s: String) -> bool {
    !s.is_empty() && !s.contains(' ')
}
fn is_short(s: String) -> bool {
    s.chars().count() < 3
}

fn is_mostly_numeric(s: String) -> bool {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return false;
    }

    let digit_count = trimmed.chars().filter(|c| c.is_ascii_digit()).count();
    let total_count = trimmed.chars().filter(|c| !c.is_whitespace()).count();

    digit_count as f64 / total_count as f64 > 0.5
}

fn char_width(s: String, total_width: u32) -> u32 {
    let len = s.chars().count();
    if len == 0 {
        return 0;
    }
    total_width / (len as u32)
}

fn avg_char_width(lines: &Vec<Line>) -> u32 {
    let mut width = 0;
    for line in lines.clone() {
        width += char_width(line.text, line.bbox.width);
    }

    width / (lines.len()) as u32
}

/// 判断字符串是否包含数学符号
fn contains_math_symbols(s: String) -> bool {
    s.chars().any(|c| {
        // 检查是否属于数学符号Unicode分类
        c.is_math_symbol() || c.is_math_operator()
    })
}

/// 扩展方法：判断字符是否为数学符号
trait MathSymbolCheck {
    fn is_math_symbol(&self) -> bool;
    fn is_math_operator(&self) -> bool;
}

impl MathSymbolCheck for char {
    fn is_math_symbol(&self) -> bool {
        match *self {
            // 基本数学运算符
            '+' | '-' | '*' | '/' | '=' | '±' | '×' | '÷' => true,
            // 比较运算符
            '<' | '>' | '≤' | '≥' | '≠' => true,
            // 括号
            '(' | ')' | '[' | ']' | '{' | '}' => true,
            // 指数和根号
            '^' | '√' | '∛' | '∜' => true,
            // 其他常见数学符号
            '∑' | '∏' | '∫' | '∂' | '∞' | 'π' | 'Δ' => true,
            _ => false,
        }
    }

    fn is_math_operator(&self) -> bool {
        matches!(
            *self,
            '+' | '-' | '*' | '/' | '=' | '±' | '×' | '÷' | '<' | '>' | '≤' | '≥' | '≠'
        )
    }
}
