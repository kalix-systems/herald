use super::*;

impl Messages {
    pub(crate) fn body_(
        &self,
        index: usize,
    ) -> Option<String> {
        let elider = &self.elider;
        let pattern = &self.search.pattern;
        let match_status = self.container.get(index).as_ref()?.match_status;

        let body: String = self
            .container
            .access_by_index(index, |data| data.text().map(ToString::to_string))
            .flatten()?;

        if match_status.is_match() {
            messages_helper::search::highlight_message(pattern.as_ref()?, &body).into()
        } else {
            elider.elided_body(body).into()
        }
    }

    pub(crate) fn full_body_(
        &self,
        index: usize,
    ) -> Option<String> {
        let pattern = &self.search.pattern;
        let match_status = self.container.get(index).as_ref()?.match_status;

        self.container.access_by_index(index, |data| {
            if match_status.is_match() {
                Some(messages_helper::search::highlight_message(
                    pattern.as_ref()?,
                    data.text()?,
                ))
            } else {
                data.text().map(ToString::to_string)
            }
        })?
    }

    pub(crate) fn set_elision_line_count_(
        &mut self,
        line_count: u8,
    ) {
        self.elider.set_line_count(line_count as usize);
    }

    pub(crate) fn set_elision_char_count_(
        &mut self,
        char_count: u16,
    ) {
        self.elider.set_char_count(char_count as usize);
    }

    pub(crate) fn set_elision_chars_per_line_(
        &mut self,
        chars_per_line: u8,
    ) {
        self.elider.set_char_per_line(chars_per_line as usize);
    }
}
