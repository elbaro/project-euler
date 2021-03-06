// e100 = [2; 1,2,1, 1,4,1, 1,6,1, .. 1, 66, 1]
use num::integer::Integer;
use num_bigint::BigInt;
use num_traits::identities::{One, Zero};

fn main() {
    let mut seq = vec![0; 100];
    seq[0] = 2;
    {
        let mut idx = 1;
        for i in 1..34 {
            seq[idx] = 1;
            seq[idx + 1] = i * 2;
            seq[idx + 2] = 1;
            idx += 3;
        }
    }
    seq = seq.into_iter().rev().collect();

    let mut f = (BigInt::one(), BigInt::zero());
    for &d in &seq[0..] {
        f = (f.1, f.0);
        f.0 += BigInt::from(d) * &f.1;
        let g = f.0.gcd(&f.1);
        f.0 /= &g;
        f.1 /= &g;
    }
    let ans: u32 =
        f.0.to_string()
            .chars()
            .map(|ch| (ch as u8 - '0' as u8) as u32)
            .sum();
    println!("{}", ans);
}
