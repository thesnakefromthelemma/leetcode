// Unfortunately can't use #[proc_macro]
// >>> primes :: [Int]; primes = 2 : filter (\ n -> and . map (not . (0 ==) . rem n) $ takeWhile ((n >=) . (^ (2 :: Int))) primes) [3..]
// >>> takeWhile ((100000 >=) . (^ (2 :: Int))) primes
const SMALL_PRIMES_LENGTH: usize = 65;
const SMALL_PRIMES: [i32; 65] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313,
];

// Quite fast, but at what cost?
pub fn sum_four_divisors(nums: &[i32]) -> i32 {
    let mut sum = 0;

    for &num in nums {
        // First we verify that num has at least 1 prime divisor
        if 1 < num {
            // Now we look for the smallest prime divisor of num
            'first_prime_search: for i in 0..SMALL_PRIMES_LENGTH {
                let prime = SMALL_PRIMES[i];

                // In this case num is a prime
                if num <= prime * prime {
                    break 'first_prime_search;
                }
                // If prime doesn't divide num we can ignore all the following
                else if 0 != num % prime {
                    continue 'first_prime_search;
                }
                // If num is prime's cube we add its divisors
                else if num == prime * prime * prime {
                    sum += num + prime * prime + prime + 1;
                    break 'first_prime_search;
                }
                // Otherwise it better not be divisible by prime's square
                else if 0 == num % (prime * prime) {
                    break 'first_prime_search;
                }
                // Now we know that num has prime as its smallest prime divisor with multiplicity 1
                else {
                    let coprime = num / prime;

                    // So we see if the quotient is also prime
                    for j in (i + 1)..SMALL_PRIMES_LENGTH {
                        let prime_prime = SMALL_PRIMES[j];

                        // In these cases it is
                        if coprime < prime_prime * prime_prime || coprime == prime_prime {
                            sum += num + prime + coprime + 1;
                            break 'first_prime_search;
                        }
                        // And in this one it isn't
                        else if 0 == coprime % prime_prime {
                            break 'first_prime_search;
                        } else {
                        }
                    }

                    // If we end up here it is, by exhausting SMALL_PRIMES
                    sum += num + prime + coprime + 1;

                    break 'first_prime_search;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::sum_four_divisors;

    #[test]
    fn test1() {
        assert_eq!(sum_four_divisors(&vec![21, 4, 7][..]), 32)
    }

    #[test]
    fn test2() {
        assert_eq!(sum_four_divisors(&vec![21, 21][..]), 64)
    }

    #[test]
    fn test3() {
        assert_eq!(sum_four_divisors(&vec![1, 2, 3, 4, 5][..]), 0)
    }
}
