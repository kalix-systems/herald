use super::*;
use heraldcore::errors::HErr;

impl MessageSearch {
    pub(super) fn start_search(&mut self, pattern: SearchPattern) -> Result<(), HErr> {
        let (tx, rx) = unbounded();

        let mut emit = self.emit.clone();
        std::thread::Builder::new().spawn(move || -> Option<()> {
            let mut searcher = Search::new(pattern);

            while let Some(results) = ret_err!(searcher.next_page(), None) {
                if results.is_empty().not() {
                    tx.send(results).ok()?;
                    emit.new_data_ready();
                }
            }

            Some(())
        })?;

        self.rx.replace(rx);

        Ok(())
    }
}
