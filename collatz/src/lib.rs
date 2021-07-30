#[inline]
fn calc_next_collatz(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

#[inline]
pub fn collatz(n: u64, steps: u64) -> u64 {
    let res = calc_next_collatz(n);
    //println!("Next collatz number: {}", n);
    //println!{"Step {}",steps};
    if res == 1 {
        steps + 1
    } else {
        collatz(res, steps + 1)
    }
}