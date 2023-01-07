use crate::partitionings::Debt;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Transaction {
    pub payer: String,
    pub payee: String,
    pub value: u32,
}

pub fn balance_debts(debts: &[&Debt]) -> Vec<Transaction> {
    let debts_count = debts.len();
    if debts_count == 0 {
        return vec![];
    }

    let mut debts: Vec<_> = debts.iter().collect();
    debts.sort_by(|a, b| a.value.cmp(&b.value));
    let mut transactions = vec![];
    let mut payee_index = 0_usize;
    let mut payer_index = debts_count - 1;
    while payee_index < payer_index {
        let payee = debts[payee_index];
        let payer = debts[payer_index];
        let mut transaction = Transaction {
            payer: payer.name.clone(),
            payee: payee.name.clone(),
            value: 0,
        };
        match payee.value.abs().cmp(&payer.value) {
            Ordering::Less => {
                transaction.value = -payee.value as u32;
                payee_index += 1;
            }
            Ordering::Equal => {
                transaction.value = payer.value as u32;
                payee_index += 1;
                payer_index -= 1;
            }
            Ordering::Greater => {
                transaction.value = payer.value as u32;
                payer_index -= 1;
            }
        }
        transactions.push(transaction);
    }
    transactions
}

mod tests {
    use super::*;

    #[test]
    fn test_balance_debts() {
        let debts = vec![
            Debt {
                name: String::from("a"),
                value: -6,
            },
            Debt {
                name: String::from("d"),
                value: 2,
            },
            Debt {
                name: String::from("f"),
                value: 4,
            },
        ];
        let partition: Vec<_> = debts.iter().collect();
        assert_eq!(balance_debts(&partition).len(), 2);
    }
}
