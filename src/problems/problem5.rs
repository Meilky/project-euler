pub fn solve() -> usize {
    let mut i = 6;
    let mut found = false;

    'top: while !found {
        for j in 2..21 {
            if i % j != 0 {
                i += j - 1;
                continue 'top;
            }
        }

        found = true;
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem5() {
        assert_eq!(solve(), 232792560);
    }
}
