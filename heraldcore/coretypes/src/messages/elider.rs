use unicode_segmentation::UnicodeSegmentation;

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
        body: String,
    ) -> String {
        let graphemes = UnicodeSegmentation::graphemes(body.as_str(), true);

        let mut char_count = 0;
        let mut line_count = 0;

        for s in graphemes {
            if char_count >= self.char_count || line_count >= self.line_count {
                break;
            }

            char_count += 1;
            line_count += s.lines().count().saturating_sub(1);
        }

        if char_count < self.char_count && line_count < self.line_count {
            return body;
        }

        let chars_to_take = self.char_count.min(self.line_count * self.char_per_line);

        let mut out: String = UnicodeSegmentation::graphemes(body.as_str(), true)
            .take(chars_to_take)
            .collect();

        out.push_str("...");

        out
    }
}
