/*
Multiples of 3 and 5
Problem 1

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/
use std::collections::HashSet;

fn main() {
    let mut multiples: HashSet<usize> = HashSet::new();

    let mut three = 3;
    let mut five = 5;

    loop {
        if three >= 1000 {
            break
        }
        multiples.insert(three);
        three += 3;
    }
    loop {
        if five >= 1000 {
            break
        }
        multiples.insert(five);
        five += 5;
    }

    dbg!(multiples.into_iter().sum::<usize>());
}
