use std::iter::Sum;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Debt {
    pub name: String,
    pub value: i32,
}

impl<'a> Sum<&'a Debt> for i32 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Debt>,
    {
        iter.map(|item| item.value).sum()
    }
}
