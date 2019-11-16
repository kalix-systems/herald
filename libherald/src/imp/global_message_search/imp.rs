use super::*;
use heraldcore::errors::HErr;

pub(super) fn start_search(
    pattern: SearchPattern,
    emit: &mut Emitter,
) -> Result<Receiver<Vec<SearchResult>>, HErr> {
    let mut emit = emit.clone();

    let (tx, rx) = unbounded();
    std::thread::Builder::new().spawn(move || -> Option<()> {
        let mut searcher = Search::new(pattern);

        while let Some(results) = ret_err!(searcher.next_page(), None) {
            tx.send(results).ok()?;
            emit.new_data_ready();
        }

        Some(())
    })?;

    Ok(rx)
}
