use std::cmp::Ordering::{Equal, Greater, Less};

// 6 * [1, 1] [2, 2; 2, 3] ^ {n - 1} [1; 1]
// (Coeffs computed w/ GHCi. Sadly the charpoly doesn't split mod p...)
pub fn num_of_ways(n: i32) -> i32 {
    match 1.cmp(&n) {
        Greater => 0,
        Equal => 12,
        Less => {
            // Modulus from problem statement
            // (must be 64, not 32, bits)
            let p = u64::pow(10, 9) + 7;

            // Exponent
            let mut pow = n - 1;

            // Symmetric matrix being iteratively squared
            let mut a_pow = 2;
            let mut b_pow = 2;
            let mut c_pow = 3;

            // Accumulated product
            let mut a_tot = 1;
            let mut b_tot = 0;
            let mut c_tot = 1;

            while 1 < pow {
                if pow % 2 == 1 {
                    let a_tot_new = (a_pow * a_tot + b_pow * b_tot) % p;
                    let b_tot_new = (a_pow * b_tot + b_pow * c_tot) % p;
                    let c_tot_new = (c_pow * c_tot + b_pow * b_tot) % p;
                    a_tot = a_tot_new;
                    b_tot = b_tot_new;
                    c_tot = c_tot_new;
                }

                let a_pow_new = (a_pow * a_pow + b_pow * b_pow) % p;
                let b_pow_new = ((a_pow + c_pow) * b_pow) % p;
                let c_pow_new = (c_pow * c_pow + b_pow * b_pow) % p;
                a_pow = a_pow_new;
                b_pow = b_pow_new;
                c_pow = c_pow_new;

                pow >>= 1;
            }

            // It's now guaranteed that pow == 1
            let a_tot_new = (a_pow * a_tot + b_pow * b_tot) % p;
            let b_tot_new = (a_pow * b_tot + b_pow * c_tot) % p;
            let c_tot_new = (c_pow * c_tot + b_pow * b_tot) % p;
            a_tot = a_tot_new;
            b_tot = b_tot_new;
            c_tot = c_tot_new;

            ((6 * a_tot + 12 * b_tot + 6 * c_tot) % p) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::num_of_ways;

    #[test]
    fn test1() {
        assert_eq!(num_of_ways(1), 12);
    }

    #[test]
    fn test2() {
        assert_eq!(num_of_ways(5000), 30228214);
    }
}
