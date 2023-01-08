use crate::partitionings::Debt;
use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Transaction {
    pub payer: String,
    pub payee: String,
    pub value: i32,
}

pub fn pay_credited(debts: &[&Debt]) -> Vec<Transaction> {
    if debts.is_empty() {
        return vec![];
    }

    let mut sorted_debts: VecDeque<_> = debts.iter().map(|&debt| debt.clone()).collect();
    sorted_debts
        .make_contiguous()
        .sort_by_key(|item| item.value);
    let mut transactions = vec![];
    while !sorted_debts.is_empty() {
        let mut creditor = sorted_debts.pop_front().unwrap();
        let mut debtor = sorted_debts.pop_back().unwrap();
        let mut transaction = Transaction {
            payer: debtor.name.clone(),
            payee: creditor.name.clone(),
            value: 0,
        };
        match creditor.value.abs().cmp(&debtor.value) {
            Ordering::Less => {
                transaction.value = -creditor.value;
                debtor.value += creditor.value;
                sorted_debts.push_back(debtor);
            }
            Ordering::Equal => {
                transaction.value = debtor.value;
            }
            Ordering::Greater => {
                transaction.value = debtor.value;
                creditor.value += debtor.value;
                sorted_debts.push_front(creditor);
            }
        }
        transactions.push(transaction);
        sorted_debts
            .make_contiguous()
            .sort_by_key(|item| item.value);
    }
    transactions
}

pub fn pay_debted(debts: &[&Debt]) -> Vec<Transaction> {
    if debts.is_empty() {
        return vec![];
    }
    let mut sorted_debts: VecDeque<_> = debts.iter().map(|&debt| debt.clone()).collect();
    sorted_debts
        .make_contiguous()
        .sort_by_key(|item| item.value);
    let mut transactions = vec![];
    while !sorted_debts.is_empty() {
        let mut creditor = sorted_debts.pop_front().unwrap();
        let debtor = sorted_debts.pop_back().unwrap();
        let mut transaction = Transaction {
            payer: debtor.name.clone(),
            payee: creditor.name.clone(),
            value: 0,
        };
        match creditor.value.abs().cmp(&debtor.value) {
            Ordering::Less => {
                transaction.value = debtor.value;
                creditor.value += debtor.value;
                sorted_debts.push_back(creditor);
            }
            Ordering::Equal => {
                transaction.value = debtor.value;
            }
            Ordering::Greater => {
                transaction.value = debtor.value;
                creditor.value += debtor.value;
                sorted_debts.push_front(creditor);
            }
        }
        transactions.push(transaction);
        sorted_debts
            .make_contiguous()
            .sort_by_key(|item| item.value);
    }
    transactions
}

mod tests {
    use super::*;

    #[test]
    fn test_pay_credited() {
        let debts: Vec<_> = [-6, -1, 3, 4]
            .into_iter()
            .enumerate()
            .map(|(i, value)| Debt {
                name: (('a' as u8 + i as u8) as char).to_string(),
                value,
            })
            .collect();
        let partition: Vec<_> = debts.iter().collect();
        let transactions = pay_credited(&partition);
        assert_eq!(
            transactions,
            [
                Transaction {
                    payer: String::from("d"),
                    payee: String::from("a"),
                    value: 4,
                },
                Transaction {
                    payer: String::from("c"),
                    payee: String::from("a"),
                    value: 2,
                },
                Transaction {
                    payer: String::from("c"),
                    payee: String::from("b"),
                    value: 1,
                },
            ]
        );
    }

    #[test]
    fn test_pay_debted() {
        let debts: Vec<_> = [4000, 2000, 1090, 1000, -1080, -1340, -2410, -3260]
            .into_iter()
            .enumerate()
            .map(|(i, value)| Debt {
                name: (('a' as u8 + i as u8) as char).to_string(),
                value,
            })
            .collect();
        let partition: Vec<_> = debts.iter().collect();
        let transactions = pay_debted(&partition);
        assert_eq!(
            transactions,
            [
                Transaction {
                    payer: String::from("a"),
                    payee: String::from("h"),
                    value: 4000,
                },
                Transaction {
                    payer: String::from("b"),
                    payee: String::from("g"),
                    value: 2000,
                },
                Transaction {
                    payer: String::from("c"),
                    payee: String::from("f"),
                    value: 1090,
                },
                Transaction {
                    payer: String::from("d"),
                    payee: String::from("e"),
                    value: 1000,
                },
                Transaction {
                    payer: String::from("h"),
                    payee: String::from("g"),
                    value: 740,
                },
                Transaction {
                    payer: String::from("g"),
                    payee: String::from("f"),
                    value: 330,
                },
                Transaction {
                    payer: String::from("f"),
                    payee: String::from("e"),
                    value: 80,
                },
            ]
        )
    }
}
