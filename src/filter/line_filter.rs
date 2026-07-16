use crate::format::format::Line;

pub struct Filter {
    pub results: Vec<Line>,
}

impl Filter {
    pub fn font_filter(&self) -> Vec<Line> {
        let mut _lines: Vec<Line> = Vec::new();
        let width = avg_char_width(&self.results);
        for line in &self.results {
            if is_short(line.text.clone()) {
                continue;
            }
            if is_single_word(line.text.clone()) {
                continue;
            }
            if is_mostly_numeric(line.text.clone()) {
                continue;
            }
            if char_width(line.text.clone(), line.bbox.width)+2<width {
                continue;
            }
            _lines.push(Line {
                index: line.index,
                bbox: line.bbox,
                text: line.text.clone(),
            });
        }
        _lines
    }
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
