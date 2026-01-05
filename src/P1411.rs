use std::cmp::Ordering::{Equal, Greater, Less};

// Modulus from problem statement
// It's critical that 7 * P * P < 2 ^ 63
const P: i64 = i64::pow(10, 9) + 7;

// 6 * [1, 1] [2, 2; 2, 3] ^ {n - 1} [1; 1]
// (Coeffs computed w/ GHCi. Sadly the charpoly doesn't split mod p...)
pub fn num_of_ways(n: i32) -> i32 {
    match 1.cmp(&n) {
        Greater => 0,
        Equal => 12,
        Less => {
            // Exponent
            let mut exp = n - 1;

            // Iteratively squared matrix as lincomb of [1, 0; 0, 1] and [2, 2; 2, 3]
            let mut a_pow = 0;
            let mut b_pow = 1;

            // Accumulated product matrix as lincomb of [1, 0; 0, 1] and [2, 2; 2, 3]
            let mut a_tot = 1;
            let mut b_tot = 0;

            while 1 < exp {
                if 1 & exp == 1 {
                    // Multiply iteratively squared matrix and accumulator
                    let a_tot_new = (a_pow * a_tot - 2 * b_pow * b_tot) % P;
                    let b_tot_new = (a_pow * b_tot + b_pow * a_tot + 5 * b_pow * b_tot) % P;
                    a_tot = a_tot_new;
                    b_tot = b_tot_new;
                }

                // Square iteratively squared matrix
                let a_pow_new = (a_pow * a_pow - 2 * b_pow * b_pow) % P;
                let b_pow_new = ((2 * a_pow + 5 * b_pow) * b_pow) % P;
                a_pow = a_pow_new;
                b_pow = b_pow_new;

                exp >>= 1;
            }

            // Now guaranteed that exp == 1
            let a_tot_new = (a_pow * a_tot - 2 * b_pow * b_tot) % P;
            let b_tot_new = (a_pow * b_tot + b_pow * a_tot + 5 * b_pow * b_tot) % P;
            a_tot = a_tot_new;
            b_tot = b_tot_new;

            // Sign issues
            ((12 * a_tot + 54 * b_tot + 66 * P) % P) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::num_of_ways;

    #[test]
    fn test1() {
        assert_eq!(num_of_ways(1), 12)
    }

    #[test]
    fn test2() {
        assert_eq!(num_of_ways(5000), 30228214)
    }
}
