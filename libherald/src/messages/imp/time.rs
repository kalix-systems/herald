use super::*;

impl Messages {
    pub(crate) fn insertion_time_(
        &self,
        index: usize,
    ) -> Option<i64> {
        Some(self.container.get(index)?.insertion_time.into())
    }

    pub(crate) fn expiration_time_(
        &self,
        index: usize,
    ) -> Option<i64> {
        self.container.access_by_index(index, |data| {
            data.time.expiration.map(herald_common::Time::into)
        })?
    }

    pub(crate) fn server_time_(
        &self,
        index: usize,
    ) -> Option<i64> {
        self.container.access_by_index(index, |data| {
            data.time.server.map(herald_common::Time::into)
        })?
    }
}
