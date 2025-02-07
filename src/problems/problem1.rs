pub fn solve() -> usize {
    (1..1000)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_problem_1() {
        assert_eq!(solve(), 233168);
    }
}
