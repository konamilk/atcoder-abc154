use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

use itertools_num::ItertoolsNum;

fn main() {
    let source = AutoSource::from("5 3
    1 2 2 4 5");
    input!{
        // from source,
        n: usize,
        k: usize,
        p: [f64; n]
    };

    let exp = p.iter().map(|&pi| pi*(pi+1.0)/2.0 / pi).collect::<Vec<f64>>();

    // println!("{:?}", exp);

    let mut s = exp.iter().cumsum().collect::<Vec<_>>();
    s.insert(0, 0.0);

    // println!("{:?}", s);

    let mut ans = std::f64::MIN;

    for i in 0..n+1-k{
        if s[i+k] - s[i]  > ans {
            ans = s[i+k] - s[i];
        }
    }

    println!("{}", ans);

}
