use std::rc::Rc;
use yew::prelude::*;

use crate::partitionings::Debt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub debts: Vec<Debt>,
}

pub enum Action {
    Add,
    Remove(usize),
    Edit((usize, String, i32)),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut debts = self.debts.clone();
        match action {
            Action::Add => {
                debts.push(Debt::new());
                State { debts }.into()
            }
            Action::Remove(i) => {
                debts.remove(i);
                if debts.is_empty() {
                    debts.push(Debt::new());
                }
                State { debts }.into()
            }
            Action::Edit((i, name, value)) => {
                debts[i] = Debt { name, value };
                State { debts }.into()
            }
        }
    }
}
