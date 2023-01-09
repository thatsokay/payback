#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Debt {
    pub name: String,
    pub value: i32,
}

impl Debt {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            value: 0,
        }
    }

    pub fn from(name: &str, value: i32) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
}

pub fn longest_zero_sum_partitionings(set: &[Debt]) -> Vec<Vec<Vec<&Debt>>> {
    if set.is_empty() {
        return vec![vec![]];
    }
    let mut subset_bit_strings: Vec<_> = (1..2_u64.pow(set.len() as u32)).collect();
    subset_bit_strings.sort_by_key(|item| item.count_zeros());
    let mut zero_sum_subset_bit_strings = vec![];
    while let Some(bit_string) = subset_bit_strings.pop() {
        let subset_sum: i32 = set
            .iter()
            .enumerate()
            .map(|(index, item)| {
                if (bit_string >> index) & 1 == 1 {
                    item.value
                } else {
                    0
                }
            })
            .sum();
        if subset_sum == 0 {
            subset_bit_strings
                .drain_filter(|subset_bit_string| *subset_bit_string & bit_string == bit_string);
            zero_sum_subset_bit_strings.push(bit_string);
        }
    }

    if zero_sum_subset_bit_strings.is_empty() {
        return vec![];
    }
    let set_bit_string = 2_u64.pow(set.len() as u32) - 1;
    let mut subset_set_bit_strings: Vec<_> =
        (1..2_u64.pow(zero_sum_subset_bit_strings.len() as u32)).collect();
    subset_set_bit_strings.sort_by_key(|item| item.count_zeros());
    let longest_partitionings = subset_set_bit_strings
        .group_by(|a, b| a.count_zeros() == b.count_zeros())
        .find_map(|group| {
            let partitionings: Vec<_> = group
                .iter()
                .filter_map(|&subset_set_bit_string| {
                    let subset_bit_strings: Vec<_> = zero_sum_subset_bit_strings
                        .iter()
                        .enumerate()
                        .filter(|(index, _)| (subset_set_bit_string >> *index) & 1 == 1)
                        .map(|(_, &bit_string)| bit_string)
                        .collect();
                    if subset_bit_strings.iter().sum::<u64>() == set_bit_string
                        && subset_bit_strings
                            .iter()
                            .fold(0, |acc, bit_string| acc | bit_string)
                            == set_bit_string
                    {
                        Some(subset_bit_strings)
                    } else {
                        None
                    }
                })
                .map(|subset_bit_strings| {
                    subset_bit_strings
                        .into_iter()
                        .map(|subset_bit_string| {
                            set.iter()
                                .enumerate()
                                .filter(|(index, _)| (subset_bit_string >> index) & 1 == 1)
                                .map(|(_, partition)| partition)
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>()
                })
                .collect();
            if partitionings.is_empty() {
                None
            } else {
                Some(partitionings)
            }
        });
    match longest_partitionings {
        None => vec![],
        Some(partitionings) => partitionings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_zero_sum_partitionings_single_result() {
        let length = 10;
        let set: Vec<_> = (0..length)
            .flat_map(|i| {
                vec![
                    Debt {
                        name: ((i as u8 + 'a' as u8) as char).to_string(),
                        value: 2_i32.pow(i),
                    },
                    Debt {
                        name: ((i as u8 + length as u8 + 'a' as u8) as char).to_string(),
                        value: -2_i32.pow(i),
                    },
                ]
            })
            .collect();
        let partitionings = longest_zero_sum_partitionings(&set);
        assert_eq!(partitionings.len(), 1);
        assert_eq!(partitionings[0].len(), length as usize);
    }

    #[test]
    fn test_longest_zero_sum_partitionings_with_multiple_results() {
        let set: Vec<_> = [-6, -2, -1, 2, 3, 4]
            .into_iter()
            .enumerate()
            .map(|(i, value)| Debt {
                name: ((i as u8 + 'a' as u8) as char).to_string(),
                value,
            })
            .collect();
        let partitionings = longest_zero_sum_partitionings(&set);
        assert_eq!(partitionings.len(), 2);
        assert_eq!(partitionings[0].len(), 2);
        assert_eq!(partitionings[1].len(), 2);
    }
}
