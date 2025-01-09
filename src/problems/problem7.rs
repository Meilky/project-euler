pub fn solve() -> usize {
    let mut nb_primes = 2;
    let mut primes: Vec<usize> = vec![1, 2];
    let mut i = 3;

    'top: while nb_primes != 25 {
        for k in 1..primes.len() {
            let prime = primes[k];

            if i % prime == 0 {
                i += primes[k - 1];
                continue 'top;
            }
        }

        primes.push(i);
        i += 1;
        nb_primes += 1;
    }

    println!("{:?}", primes);

    primes[primes.len()-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem7() {
        assert_eq!(solve(), 104743);
    }
}
