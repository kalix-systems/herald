use super::*;

impl Messages {
    pub(crate) fn doc_attachments_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container.doc_attachments_data_json(index, None)
    }

    pub(crate) fn media_attachments_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container.media_attachments_data_json(index, 4.into())
    }

    pub(crate) fn full_media_attachments_(
        &self,
        index: usize,
    ) -> Option<String> {
        self.container.media_attachments_data_json(index, None)
    }

    pub(crate) fn save_all_attachments_(
        &self,
        index: usize,
        dest: String,
    ) -> bool {
        let dest = none!(crate::utils::strip_qrc(dest), false);
        let data = none!(self.container.access_by_index(index, MsgData::clone), false);

        spawn!(err!(data.save_all_attachments(dest)), false);
        true
    }
}
