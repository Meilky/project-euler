pub fn solve() -> usize {
    let sum_squares: usize = (1..101).map(|v: usize| v.pow(2)).sum();

    let square_sum: usize = (1..101).sum::<usize>().pow(2);


    square_sum - sum_squares
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_problem_6() {
        assert_eq!(solve(), 25164150);
    }
}
