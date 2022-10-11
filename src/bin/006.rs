//! ## Sum square difference
//!
//! Find the difference between the sum of the squares of the first one hundred
//! natural numbers and the square of the sum.

fn main() {
	dbg!((1..=100).sum::<u64>().pow(2) - (1..=100).map(|x: u64| x.pow(2)).sum::<u64>());
}
