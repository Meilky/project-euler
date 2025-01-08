pub fn solve() -> usize {
    let mut a: usize = 1;
    let mut b: usize = 2;

    let mut total: usize = 2;

    while b <= 4_000_000 {
        let sum = a + b;

        if sum % 2 == 0 {
            total += sum;
        }

        a = b;
        b = sum;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem2() {
        assert_eq!(solve(), 4613732);
    }
}
