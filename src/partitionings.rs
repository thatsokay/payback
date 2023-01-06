use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Debt {
    pub name: String,
    pub value: i32,
}

/// https://stackoverflow.com/a/20530130
pub fn all_partitionings<T: Copy + Eq + PartialEq + Hash>(set: HashSet<T>) -> Vec<Vec<HashSet<T>>> {
    if set.is_empty() {
        return vec![vec![]];
    }
    (0..2_u32.pow(set.len() as u32 - 1))
        .flat_map(|inclusion_bit_string| -> Vec<Vec<HashSet<_>>> {
            let mut fixed_subset = HashSet::new();
            let mut recurse_subset = HashSet::new();
            for (index, &item) in set.iter().enumerate() {
                if inclusion_bit_string & (1 << index) == 0 {
                    fixed_subset.insert(item);
                } else {
                    recurse_subset.insert(item);
                }
            }
            all_partitionings(recurse_subset)
                .into_iter()
                .map(|mut partitioning| {
                    partitioning.push(fixed_subset.clone());
                    partitioning
                })
                .collect()
        })
        .collect()
}

pub fn zero_sum_partitionings(debts: HashSet<&Debt>) -> Vec<Vec<HashSet<&Debt>>> {
    all_partitionings(debts)
        .into_iter()
        .filter(|partitioning| {
            partitioning.iter().all(|partitioning| {
                partitioning
                    .iter()
                    .map(|partition| partition.value)
                    .sum::<i32>()
                    == 0
            })
        })
        .collect()
}

pub fn longest_items<T>(xs: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let max_length = xs.iter().map(|x| x.len()).max().unwrap();
    xs.into_iter().filter(|x| x.len() == max_length).collect()
}

pub fn all_subsets<T>(set: &[T], size: usize) -> Vec<Vec<&T>> {
    if size == 0 {
        return vec![vec![]];
    }
    set.iter()
        .enumerate()
        .flat_map(|(index, item)| {
            all_subsets(&set[(index + 1)..], size - 1)
                .into_iter()
                .map(|mut subset| {
                    subset.push(item);
                    subset
                })
        })
        .collect()
}

// /// https://stackoverflow.com/a/20530130
// pub fn zero_sum_partitionings_2(set: &HashSet<Debt>) -> Vec<Vec<HashSet<&Debt>>> {
//     if set.is_empty() {
//         return vec![vec![]];
//     }
//     (0..2_u32.pow(set.len() as u32 - 1))
//         .flat_map(|inclusion_bit_string| -> Vec<Vec<HashSet<_>>> {
//             let mut fixed_subset = HashSet::new();
//             let mut recurse_subset = HashSet::new();
//             for (index, item) in set.iter().enumerate() {
//                 if inclusion_bit_string & (1 << index) == 0 {
//                     fixed_subset.insert(item);
//                 } else {
//                     recurse_subset.insert(item);
//                 }
//             }
//             println!("{:?}", fixed_subset);
//             if fixed_subset.iter().map(|item| item.value).sum::<i32>() != 0 {
//                 return vec![];
//             }
//             zero_sum_partitionings_2(&recurse_subset)
//                 .into_iter()
//                 .map(|mut partitioning| {
//                     partitioning.push(fixed_subset.clone());
//                     partitioning
//                 })
//                 .collect()
//         })
//         .collect()
// }

pub fn zero_sum_partitionings_3<'a>(set: Vec<&'a Debt>) -> Vec<Vec<Vec<&'a Debt>>> {
    if set.is_empty() {
        return vec![vec![]];
    }
    let mut inclusion_bit_strings: Vec<_> = (1..2_u32.pow(set.len() as u32)).collect();
    inclusion_bit_strings.sort_by(|a, b| match a.count_zeros().cmp(&b.count_zeros()) {
        Ordering::Equal => a.cmp(b),
        ord => ord,
    });
    inclusion_bit_strings.drain(1..=(inclusion_bit_strings.len() / 2));
    println!("");
    println!("Set {:?}", set);
    println!("Subsets {:?}", inclusion_bit_strings);
    let mut result = vec![];
    while let Some(inclusion_bit_string) = inclusion_bit_strings.pop() {
        println!(
            "Subset {:#010b} {}",
            inclusion_bit_string, inclusion_bit_string
        );
        let mut subset = vec![];
        let mut remainder = vec![];
        for (index, &item) in set.iter().enumerate() {
            if inclusion_bit_string & (1 << index) == 0 {
                remainder.push(item);
            } else {
                subset.push(item);
            }
        }
        if subset.iter().map(|item| item.value).sum::<i32>() != 0 {
            continue;
        }
        {
            // Manual `drain_filter`
            let mut i = 0;
            while i < inclusion_bit_strings.len() {
                if (inclusion_bit_strings[i] & inclusion_bit_string) == inclusion_bit_string {
                    println!(
                        "Remove {:#010b} {}",
                        inclusion_bit_strings[i], inclusion_bit_strings[i]
                    );
                    inclusion_bit_strings.remove(i);
                } else {
                    i += 1;
                }
            }
        }
        result.extend(
            zero_sum_partitionings_3(remainder)
                .into_iter()
                .map(|mut partitioning| {
                    partitioning.push(subset.clone());
                    partitioning
                }),
        );
    }
    println!("Result {:?}", result);
    result
    // (1..=(set.len())).map(|subset_size| {
    //     all_subsets(set, subset_size)
    //         .into_iter()
    //         .filter(|subset| subset.iter().map(|item| item.value).sum::<i32>() == 0)
    // });
}

pub fn zero_sum_partitionings_4<'a>(set: Vec<&'a Debt>) -> Vec<Vec<Vec<&'a Debt>>> {
    if set.is_empty() {
        return vec![vec![]];
    }
    let mut subset_bit_strings: Vec<_> = (1..2_u64.pow(set.len() as u32)).collect();
    subset_bit_strings.sort_by(|a, b| match a.count_zeros().cmp(&b.count_zeros()) {
        Ordering::Equal => a.cmp(b),
        ord => ord,
    });
    println!("");
    println!("Set {:?}", set);
    println!("Subsets {:?}", subset_bit_strings);
    let mut zero_sum_subset_bit_strings = vec![];
    while let Some(bit_string) = subset_bit_strings.pop() {
        println!("Subset {:#010b} {}", bit_string, bit_string);
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
        if subset_sum != 0 {
            continue;
        }
        {
            // Manual `drain_filter`
            let mut i = 0;
            while i < subset_bit_strings.len() {
                if (subset_bit_strings[i] & bit_string) == bit_string {
                    println!(
                        "Remove {:#010b} {}",
                        subset_bit_strings[i], subset_bit_strings[i]
                    );
                    subset_bit_strings.remove(i);
                } else {
                    i += 1;
                }
            }
        }
        zero_sum_subset_bit_strings.push(bit_string);
    }

    if zero_sum_subset_bit_strings.is_empty() {
        return vec![];
    }
    let set_bit_string = 2_u64.pow(set.len() as u32) - 1;
    (1..2_u64.pow(zero_sum_subset_bit_strings.len() as u32))
        .filter(|subset_set_bit_string| {
            zero_sum_subset_bit_strings
                .iter()
                .enumerate()
                .filter(|(index, _)| (subset_set_bit_string >> index) & 1 == 1)
                .map(|(_, &bit_string)| bit_string)
                .sum::<u64>()
                == set_bit_string
        })
        .map(|subset_set_bit_string| {
            zero_sum_subset_bit_strings
                .iter()
                .enumerate()
                .filter(move |(i, _)| (subset_set_bit_string >> i) & 1 == 1)
                .map(|(_, &subset_bit_string)| {
                    set.iter()
                        .enumerate()
                        .filter(|(j, _)| (subset_bit_string >> j) & 1 == 1)
                        .map(|(_, &partition)| partition)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn longest_zero_sum_partitionings<'a>(set: Vec<&'a Debt>) -> Vec<Vec<Vec<&'a Debt>>> {
    if set.is_empty() {
        return vec![vec![]];
    }
    let mut subset_bit_strings: Vec<_> = (1..2_u64.pow(set.len() as u32)).collect();
    subset_bit_strings.sort_by(|a, b| a.count_zeros().cmp(&b.count_zeros()));
    println!("");
    println!("Set {:?}", set);
    println!("Subsets {:?}", subset_bit_strings);
    let mut zero_sum_subset_bit_strings = vec![];
    while let Some(bit_string) = subset_bit_strings.pop() {
        println!("Subset {:#010b} {}", bit_string, bit_string);
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
    subset_set_bit_strings.sort_by(|a, b| a.count_zeros().cmp(&b.count_zeros()));
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
                            println!("{:#010b} {}", subset_bit_string, subset_bit_string);
                            set.iter()
                                .enumerate()
                                .filter(|(index, _)| (subset_bit_string >> index) & 1 == 1)
                                .map(|(_, &partition)| partition)
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
    fn test_all_partitionings() {
        let set = HashSet::from_iter(0..6);
        let partitionings = all_partitionings(set);
        assert_eq!(partitionings.len(), 203);
    }

    #[test]
    fn test_zero_sum_partitionings() {
        let debts: Vec<_> = [-6, -2, -1, 2, 3, 4]
            .into_iter()
            .enumerate()
            .map(|(i, value)| Debt {
                name: ((i as u8 + 'a' as u8) as char).to_string(),
                value,
            })
            .collect();
        let partitionings = zero_sum_partitionings(debts.iter().collect());
        assert_eq!(partitionings.len(), 3);
    }

    #[test]
    fn test_longest_zero_sum_partitionings() {
        let debts: Vec<_> = [-6, -2, -1, 2, 3, 4]
            .into_iter()
            .enumerate()
            .map(|(i, value)| Debt {
                name: ((i as u8 + 'a' as u8) as char).to_string(),
                value,
            })
            .collect();
        let partitionings = longest_items(zero_sum_partitionings(debts.iter().collect()));
        println!("{:?}", partitionings);
        assert_eq!(partitionings.len(), 2);
    }

    #[test]
    fn test_all_subsets() {
        let set = Vec::from_iter(0..4);
        let subsets = all_subsets(&set, 2);
        assert_eq!(subsets.len(), 6);
    }

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
        let partitionings = longest_zero_sum_partitionings(set.iter().collect());
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
        let partitionings = longest_zero_sum_partitionings(set.iter().collect());
        assert_eq!(partitionings.len(), 2);
        assert_eq!(partitionings[0].len(), 2);
        assert_eq!(partitionings[1].len(), 2);
    }
}
