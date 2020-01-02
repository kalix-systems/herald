use super::*;

/// An isolated message reaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reaction {
    /// The reactionary
    pub reactionary: UserId,
    /// The text of the receipt
    pub react_content: ReactContent,
    /// The time the react arrived at the client
    pub time: Time,
}

pub type ReactContent = String;

/// A `ReactContent` with an ordered list of
/// reactionaries
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct TaggedReact {
    pub content: ReactContent,
    pub reactionaries: Vec<UserId>,
}

/// A collection of message reactions
#[derive(Clone, Debug, Default)]
pub struct Reactions {
    pub content: Vec<TaggedReact>,
}

impl Reactions {
    pub fn add(
        &mut self,
        react: ReactContent,
        reactionary: UserId,
    ) -> bool {
        match self
            .content
            .iter()
            .position(|tagged| tagged.content == react)
        {
            Some(ix) => {
                if let Some(tagged) = self.content.get_mut(ix) {
                    if !tagged.reactionaries.contains(&reactionary) {
                        tagged.reactionaries.push(reactionary);
                        return true;
                    }
                }
                false
            }
            None => {
                self.content.push(TaggedReact {
                    content: react,
                    reactionaries: vec![reactionary],
                });
                true
            }
        }
    }

    pub fn remove(
        &mut self,
        react: ReactContent,
        reactionary: UserId,
    ) -> bool {
        if let Some(ix) = self
            .content
            .iter()
            .position(|tagged| tagged.content == react)
        {
            if let Some(tagged) = self.content.get_mut(ix) {
                if let Some(position) = tagged.reactionaries.iter().position(|u| u == &reactionary)
                {
                    tagged.reactionaries.remove(position);
                    return true;
                }
            }
        };
        false
    }
}
