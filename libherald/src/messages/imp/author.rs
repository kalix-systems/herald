use super::*;

impl Messages {
    pub(crate) fn author_(
        &self,
        index: usize,
    ) -> Option<ffi::UserId> {
        self.container
            .access_by_index(index, |data| data.author.to_string())
    }

    pub(crate) fn author_color_(
        &self,
        index: usize,
    ) -> Option<u32> {
        let uid = self.container.access_by_index(index, |data| data.author)?;
        crate::users::shared::color(&uid)
    }

    pub(crate) fn author_name_(
        &self,
        index: usize,
    ) -> Option<ffi::UserId> {
        let uid = self.container.access_by_index(index, |data| data.author)?;
        crate::users::shared::name(&uid)
    }

    pub(crate) fn author_profile_picture_(
        &self,
        index: usize,
    ) -> Option<String> {
        let uid = self.container.access_by_index(index, |data| data.author)?;
        crate::users::shared::profile_picture(&uid)
    }
}
