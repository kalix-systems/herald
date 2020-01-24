use super::{shared, *};
use crate::push;

impl Conversations {
    pub(crate) fn id(
        &self,
        index: usize,
    ) -> Option<ConversationId> {
        Some(self.list.get(index)?.id)
    }
}

impl Conversations {
    pub(crate) fn inner_filter(&mut self) -> Option<()> {
        let filter = &self.filter.as_ref()?;

        let list = &mut self.list;
        for (ix, Conversation { matched, id }) in list.iter_mut().enumerate() {
            let lock = shared::conv_data().read();
            let data = cont_none!(lock.get(id));

            let new_matched = match &data.title {
                Some(title) => filter.is_match(&title),
                None => false,
            };

            if new_matched != *matched {
                *matched = new_matched;
                self.model.data_changed(ix, ix);
            }
        }

        Some(())
    }

    pub(crate) fn pairwise_inner(
        &self,
        index: usize,
    ) -> Option<bool> {
        let id = &self.list.get(index).as_ref()?.id;
        shared::pairwise(id)
    }
}

impl crate::Loadable for Conversations {
    type Error = std::io::Error;

    fn try_load(&mut self) -> Result<(), std::io::Error> {
        std::thread::Builder::new().spawn(|| {
            let mut list = Vector::new();
            for meta in err!(conversation::all_meta()).into_iter() {
                let (conv, data) = split_meta(meta);
                shared::insert_data(conv.id, data);
                list.push_back(conv);
            }

            push(ConvUpdate::Init(list));
        })?;

        Ok(())
    }

    fn loaded(&self) -> bool {
        self.loaded
    }
}
