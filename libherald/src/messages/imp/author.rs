use super::*;

impl Messages {
    pub(crate) fn author_(
        &self,
        index: usize,
    ) -> Option<ffi::UserId> {
        self.container
            .access_by_index(index, |data| data.author.to_string())
    }
}
