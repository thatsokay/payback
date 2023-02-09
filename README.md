# Payback

Settles debts within a group with the fewest number of transactions.

## Background

For a group of $n$ debtors and creditors the worst case for the minimum number
of transactions to balance the debts is $n-1$ where each person pays the next
person the amount they owe and the last person pays nobody.

The number of transactions can be reduced if the group can be partitioned into
multiple groups such that each group's total debt is zero. Then the number of
transactions required is $n-m$ where $m$ is the number of groups.

*Payback* finds the group partitionings with the most partitions to minimise the
number of transactions required to settle all debts.
