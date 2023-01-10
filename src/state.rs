use std::rc::Rc;
use yew::prelude::*;

use crate::debt::Debt;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Entry {
    pub id: usize,
    pub debt: Debt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub entries: Vec<Entry>,
}

impl State {
    pub fn new() -> Self {
        Self {
            entries: vec![Default::default()],
        }
    }
}

pub enum Action {
    Add,
    Remove(usize),
    Edit((usize, String, i32)),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut entries = self.entries.clone();
        match action {
            Action::Add => {
                entries.push(Entry {
                    id: entries.last().unwrap().id + 1,
                    debt: Default::default(),
                });
                State { entries }.into()
            }
            Action::Remove(i) => {
                entries.remove(i);
                if entries.is_empty() {
                    entries.push(Default::default());
                }
                State { entries }.into()
            }
            Action::Edit((i, name, value)) => {
                entries[i] = Entry {
                    id: entries[i].id,
                    debt: Debt { name, value },
                };
                State { entries }.into()
            }
        }
    }
}
