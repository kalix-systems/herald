use super::*;

pub(super) fn init() {
    spawn!({
        let contents = ret_err!(conversation::all_meta())
            .into_iter()
            .map(|inner| Conversation {
                inner,
                matched: true,
            })
            .collect();

        ret_err!(Conversations::push(ConvUpdate::Init(contents)));
    });
}
