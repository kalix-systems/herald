use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Elider {
    pub line_count: usize,
    pub char_count: usize,
    pub char_per_line: usize,
}

impl Default for Elider {
    fn default() -> Self {
        let line_count = 30;
        let char_per_line = 25;
        let char_count = line_count * char_per_line;

        Self {
            line_count,
            char_per_line,
            char_count,
        }
    }
}

impl Elider {
    pub fn set_line_count(
        &mut self,
        line_count: usize,
    ) {
        self.line_count = line_count
    }

    pub fn set_char_count(
        &mut self,
        char_count: usize,
    ) {
        self.char_count = char_count
    }

    pub fn set_char_per_line(
        &mut self,
        char_per_line: usize,
    ) {
        self.char_per_line = char_per_line;
    }

    pub fn elided_body(
        &self,
        body: &str,
    ) -> String {
        let body = body.trim();

        let mut char_count = 0;
        let mut line_count = 0;

        let mut out = String::new();
        let mut empty_lines = 0;

        'outer: for (ix, line) in body.lines().take(self.line_count).enumerate() {
            if char_count >= self.char_count {
                break 'outer;
            }

            let graphemes = UnicodeSegmentation::graphemes(line, true);

            if ix != 0 {
                out.push_str("\n");
            }

            let mut chars = 0;

            for s in graphemes {
                if char_count >= self.char_count {
                    break 'outer;
                }

                out.push_str(s);

                char_count += 1;
                chars += 1;
            }

            if chars == 0 {
                empty_lines += 1;
            }

            if empty_lines >= 4 {
                return out.trim_end().to_string();
            }

            line_count += 1;
        }

        if char_count < self.char_count && line_count < self.line_count {
            return body.to_string();
        }

        let mut out = out.trim_end().to_string();
        out.push_str("...");
        out
    }
}
