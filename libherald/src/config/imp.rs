use super::Config;
use herald_common::UserId;
use heraldcore::config as core;
use heraldcore::errors::HErr;

impl crate::Loadable for Config {
    type Error = HErr;

    fn try_load(&mut self) -> Result<(), HErr> {
        self.inner.replace(core::get()?);
        Ok(())
    }

    fn loaded(&self) -> bool {
        self.inner.is_some()
    }
}

impl Config {
    pub(crate) fn local_id(&self) -> Option<UserId> {
        Some(self.inner.as_ref()?.id)
    }

    pub(crate) fn handle_update(
        &mut self,
        update: super::ConfUpdate,
    ) -> Option<()> {
        use super::ConfUpdate::*;
        match update {
            Picture(maybe_path) => {
                self.inner.as_mut()?.profile_picture = maybe_path;

                self.emit.profile_picture_changed();
            }
        }

        Some(())
    }
}
