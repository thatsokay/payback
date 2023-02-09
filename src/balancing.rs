use std::cmp::Ordering;
use std::fmt;

use crate::debt::Debt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Transaction {
    pub source: String,
    pub destination: String,
    pub value: u32,
}

impl Transaction {
    pub fn from(source: &str, destination: &str, value: u32) -> Self {
        Self {
            source: source.to_string(),
            destination: destination.to_string(),
            value,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} pays ${}.{:02} to {}",
            self.source,
            self.value / 100,
            self.value % 100,
            self.destination,
        )
    }
}

/// Creates transactions from the most debted to the most credited until all
/// debts have been settled.
pub fn balance_by_debted_amounts_desc(debts: &[&Debt]) -> Vec<Transaction> {
    let mut debtors = vec![];
    let mut creditors = vec![];
    for debt in debts {
        match debt.value.cmp(&0) {
            Ordering::Less => creditors.push((*debt).clone()),
            Ordering::Equal => {}
            Ordering::Greater => debtors.push((*debt).clone()),
        }
    }
    let debtor_key_selector = |debt: &Debt| debt.value;
    let creditor_key_selector = |debt: &Debt| -debt.value;
    debtors.sort_by_key(debtor_key_selector);
    creditors.sort_by_key(creditor_key_selector);

    let mut transactions = vec![];
    while !debtors.is_empty() && !creditors.is_empty() {
        let debtor = debtors.pop().unwrap();
        let mut creditor = creditors.pop().unwrap();
        transactions.push(Transaction {
            source: debtor.name.clone(),
            destination: creditor.name.clone(),
            value: debtor.value as u32,
        });
        match (debtor.value + creditor.value).cmp(&0) {
            Ordering::Less => {
                creditor.value += debtor.value;
                let insert_index = creditors
                    .binary_search_by_key(&creditor.value, creditor_key_selector)
                    .or_else(Ok::<usize, usize>)
                    .unwrap();
                creditors.insert(insert_index, creditor);
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                creditor.value += debtor.value;
                let insert_index = debtors
                    .binary_search_by_key(&creditor.value, debtor_key_selector)
                    .or_else(Ok::<usize, usize>)
                    .unwrap();
                debtors.insert(insert_index, creditor);
            }
        }
    }
    transactions
}

/// Creates transactions from the most debted to the least credited until all
/// debts have been settled.
pub fn balance_by_debted_amounts_asc(debts: &[&Debt]) -> Vec<Transaction> {
    let mut debtors = vec![];
    let mut creditors = vec![];
    for debt in debts {
        match debt.value.cmp(&0) {
            Ordering::Less => creditors.push((*debt).clone()),
            Ordering::Equal => {}
            Ordering::Greater => debtors.push((*debt).clone()),
        }
    }
    let debt_key_selector = |debt: &Debt| debt.value;
    debtors.sort_by_key(debt_key_selector);
    creditors.sort_by_key(debt_key_selector);

    let mut transactions = vec![];
    while !debtors.is_empty() && !creditors.is_empty() {
        let debtor = debtors.pop().unwrap();
        let mut creditor = creditors.pop().unwrap();
        transactions.push(Transaction {
            source: debtor.name.clone(),
            destination: creditor.name.clone(),
            value: debtor.value as u32,
        });
        match (debtor.value + creditor.value).cmp(&0) {
            Ordering::Less => {
                creditor.value += debtor.value;
                let insert_index = creditors
                    .binary_search_by_key(&creditor.value, debt_key_selector)
                    .or_else(Ok::<usize, usize>)
                    .unwrap();
                creditors.insert(insert_index, creditor);
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                creditor.value += debtor.value;
                let insert_index = debtors
                    .binary_search_by_key(&creditor.value, debt_key_selector)
                    .or_else(Ok::<usize, usize>)
                    .unwrap();
                debtors.insert(insert_index, creditor);
            }
        }
    }
    transactions
}

/// Creates transactions from all debtors to a single person (the hub), then
/// from that person to all creditors.
pub fn balance_by_spoke_hub(debts: &[&Debt], hub_index: usize) -> Vec<Transaction> {
    let hub = debts[hub_index];
    debts[..hub_index]
        .iter()
        .chain(debts[(hub_index + 1)..].iter())
        .flat_map(|debt| match debt.value.cmp(&0) {
            Ordering::Less => Some(Transaction {
                source: hub.name.clone(),
                destination: debt.name.clone(),
                value: (-debt.value) as u32,
            }),
            Ordering::Equal => None,
            Ordering::Greater => Some(Transaction {
                source: debt.name.clone(),
                destination: hub.name.clone(),
                value: debt.value as u32,
            }),
        })
        .collect()
}

mod tests {
    use super::*;

    #[test]
    fn test_balance_by_debted_amounts_desc() {
        let debts: Vec<_> = [4000, 2000, 1090, 1000, -1080, -1340, -2410, -3260]
            .into_iter()
            .enumerate()
            .map(|(i, value)| Debt {
                name: (('a' as u8 + i as u8) as char).to_string(),
                value,
            })
            .collect();
        let partition: Vec<_> = debts.iter().collect();
        let transactions = balance_by_debted_amounts_desc(&partition);
        assert_eq!(
            transactions,
            [
                Transaction::from("a", "h", 4000),
                Transaction::from("b", "g", 2000),
                Transaction::from("c", "f", 1090),
                Transaction::from("d", "e", 1000),
                Transaction::from("h", "g", 740),
                Transaction::from("g", "f", 330),
                Transaction::from("f", "e", 80),
            ]
        )
    }

    #[test]
    fn test_balance_by_debted_amounts_asc() {
        let debts: Vec<_> = [4000, 2000, 1090, 1000, -1080, -1340, -2410, -3260]
            .into_iter()
            .enumerate()
            .map(|(i, value)| Debt {
                name: (('a' as u8 + i as u8) as char).to_string(),
                value,
            })
            .collect();
        let partition: Vec<_> = debts.iter().collect();
        let transactions = balance_by_debted_amounts_asc(&partition);
        assert_eq!(
            transactions,
            [
                Transaction::from("a", "e", 4000),
                Transaction::from("e", "f", 2920),
                Transaction::from("b", "g", 2000),
                Transaction::from("f", "g", 1580),
                Transaction::from("g", "h", 1170),
                Transaction::from("c", "h", 1090),
                Transaction::from("d", "h", 1000),
            ]
        )
    }

    #[test]
    fn test_balance_by_spoke_hub() {
        let debts: Vec<_> = [4000, 2000, 1090, 1000, -1080, -1340, -2410, -3260]
            .into_iter()
            .enumerate()
            .map(|(i, value)| Debt {
                name: (('a' as u8 + i as u8) as char).to_string(),
                value,
            })
            .collect();
        let partition: Vec<_> = debts.iter().collect();
        let transactions = balance_by_spoke_hub(&partition, 6);
        assert_eq!(
            transactions,
            [
                Transaction::from("a", "g", 4000),
                Transaction::from("b", "g", 2000),
                Transaction::from("c", "g", 1090),
                Transaction::from("d", "g", 1000),
                Transaction::from("g", "e", 1080),
                Transaction::from("g", "f", 1340),
                Transaction::from("g", "h", 3260),
            ]
        )
    }
}
