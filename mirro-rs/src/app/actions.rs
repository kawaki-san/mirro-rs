use std::collections::HashMap;
use std::fmt::{self, Display};
use std::slice::Iter;

use crate::inputs::key::Key;

use super::state::Widgets;

/// We define all available action
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
    Quit,
    Sleep,
    Focus(Widgets),
    Action,
}

impl Action {
    /// All available actions
    pub fn iterator() -> Iter<'static, Action> {
        static ACTIONS: [Action; 6] = [
            Action::Quit,
            Action::Sleep,
            Action::Focus(Widgets::CountryFilter),
            Action::Focus(Widgets::Protocols),
            Action::Focus(Widgets::Mirrors),
            Action::Action,
        ];
        ACTIONS.iter()
    }

    /// List of key associated to action
    pub fn keys(&self) -> &[Key] {
        match self {
            Action::Quit => &[Key::Ctrl('c')],
            Action::Sleep => &[Key::Ctrl('s')],
            Action::Focus(Widgets::CountryFilter) => &[Key::Ctrl('f')],
            Action::Focus(Widgets::Protocols) => &[Key::Ctrl('p')],
            Action::Focus(Widgets::Mirrors) => &[Key::Ctrl('a')],
            Action::Action => &[
                Key::Char('a'),
                Key::Char('b'),
                Key::Char('c'),
                Key::Char('d'),
                Key::Char('e'),
                Key::Char('f'),
                Key::Char('g'),
                Key::Char('h'),
                Key::Char('i'),
                Key::Char('j'),
                Key::Char('k'),
                Key::Char('l'),
                Key::Char('m'),
                Key::Char('n'),
                Key::Char('o'),
                Key::Char('p'),
                Key::Char('q'),
                Key::Char('r'),
                Key::Char('s'),
                Key::Char('t'),
                Key::Char('u'),
                Key::Char('v'),
                Key::Char('w'),
                Key::Char('x'),
                Key::Char('y'),
                Key::Char('z'),
                Key::Char(' '),
                Key::Up,
                Key::Enter,
                Key::Down,
                Key::Backspace,
                Key::Esc,
            ],
        }
    }
}

/// Could display a user friendly short description of action
impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Action::Quit => "Quit",
            Action::Sleep => "Sleep",

            Action::Focus(Widgets::CountryFilter) => "Focus Filter",
            Action::Focus(Widgets::Protocols) => "Focus Protocols",
            Action::Focus(Widgets::Mirrors) => "Focus Mirrors",
            Action::Action => "Action",
        };
        write!(f, "{}", str)
    }
}

/// The application should have some contextual actions.
#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
    /// Given a key, find the corresponding action
    pub fn find(&self, key: Key) -> Option<&Action> {
        Action::iterator()
            .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }
}

impl From<Vec<Action>> for Actions {
    /// Build contextual action
    ///
    /// # Panics
    ///
    /// If two actions have same key
    fn from(actions: Vec<Action>) -> Self {
        // Check key unicity
        let mut map: HashMap<Key, Vec<Action>> = HashMap::new();
        for action in actions.iter() {
            for key in action.keys().iter() {
                match map.get_mut(key) {
                    Some(vec) => vec.push(*action),
                    None => {
                        map.insert(*key, vec![*action]);
                    }
                }
            }
        }
        let errors = map
            .iter()
            .filter(|(_, actions)| actions.len() > 1) // at least two actions share same shortcut
            .map(|(key, actions)| {
                let actions = actions
                    .iter()
                    .map(Action::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Conflict key {} with actions {}", key, actions)
            })
            .collect::<Vec<_>>();
        if !errors.is_empty() {
            panic!("{}", errors.join("; "))
        }

        // Ok, we can create contextual actions
        Self(actions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_action_by_key() {
        let actions: Actions = vec![Action::Quit, Action::Sleep].into();
        let result = actions.find(Key::Ctrl('c'));
        assert_eq!(result, Some(&Action::Quit));
    }

    #[test]
    fn should_find_action_by_key_not_found() {
        let actions: Actions = vec![Action::Quit, Action::Sleep].into();
        let result = actions.find(Key::Alt('w'));
        assert_eq!(result, None);
    }

    #[test]
    fn should_create_actions_from_vec() {
        let _actions: Actions = vec![
            Action::Quit,
            Action::Sleep,
            Action::Focus(Widgets::CountryFilter),
            Action::Focus(Widgets::Protocols),
            Action::Focus(Widgets::Mirrors),
        ]
        .into();
    }

    #[test]
    #[should_panic]
    fn should_panic_when_create_actions_conflict_key() {
        let _actions: Actions = vec![
            Action::Quit,
            Action::Sleep,
            Action::Sleep,
            Action::Quit,
            Action::Focus(Widgets::CountryFilter),
            Action::Focus(Widgets::Protocols),
            Action::Focus(Widgets::Mirrors),
            Action::Focus(Widgets::CountryFilter),
            Action::Focus(Widgets::Protocols),
            Action::Focus(Widgets::Mirrors),
        ]
        .into();
    }
}
