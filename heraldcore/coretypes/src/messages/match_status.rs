#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchStatus {
    NotMatched = 0,
    Matched = 1,
    Focused = 2,
}

impl MatchStatus {
    pub fn is_match(self) -> bool {
        self == MatchStatus::Matched || self == MatchStatus::Focused
    }
}
