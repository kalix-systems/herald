use super::*;
use coremacros::*;

#[test]
fn test_set_pattern() {
    let mut search_state = SearchState::new();
    assert_eq!(
        search_state.set_pattern_inner("test".to_string()).unwrap(),
        SearchChanged::Changed
    );

    assert_eq!(
        search_state.set_pattern_inner("test".to_string()).unwrap(),
        SearchChanged::NotChanged
    );
}

#[test]
fn test_clear_search() {
    let mut search_state = SearchState::new();
    search_state
        .set_pattern_inner("test".to_string())
        .expect(womp!());

    search_state.clear_search_inner().expect(womp!());

    assert_eq!(search_state.pattern.unwrap().raw(), "".to_string());
}
