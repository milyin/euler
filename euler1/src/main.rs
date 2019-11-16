use std::io;
use std::io::Write;

fn sum_multiples(m : i64, n : i64) -> i64 {
    let count = (n-1) / m;
    let count_odd = count % 2;
    let count_even = if count_odd==0 {1} else {0};
    let center_value = (count + count_even)*m;
    let sum = center_value*count_odd + center_value*(count-count_odd)/2;
    sum
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut s_t = String::new();
    io::stdin().read_line(&mut s_t)?;
    let input_t : i32 = s_t.trim().parse()?;
    for _ in 0..input_t {
        let mut s_n = String::new();
        io::stdin().read_line(&mut s_n)?;
        let n : i64 = s_n.trim().parse()?;
        let sum = sum_multiples(3, n ) + sum_multiples(5, n) - sum_multiples(15, n);
        writeln!(io::stdout(), "{}", sum)?;
    }
    Ok(())
}
