use super::*;
use heraldcore::errors::HErr;

impl Users {
    pub(super) fn inner_filter(&mut self) {
        for (ix, user) in self.list.iter_mut().enumerate() {
            let inner = ret_none!(get_user(&user.id));
            let old_matched = user.matched;
            user.matched = self
                .filter
                .as_ref()
                .map(|filter| inner.matches(&filter))
                .unwrap_or(true);
            if user.matched != old_matched {
                self.model.data_changed(ix, ix);
            }
        }
    }
}

impl crate::Loadable for Users {
    type Error = HErr;

    fn try_load(&mut self) -> Result<(), HErr> {
        self.list = user::all()?
            .into_iter()
            .map(|u| {
                let id = u.id;
                shared::USER_DATA.insert(id, u);
                User { id, matched: true }
            })
            .collect();
        self.loaded = true;

        Ok(())
    }

    fn loaded(&self) -> bool {
        self.loaded
    }
}
