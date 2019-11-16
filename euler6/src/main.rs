use std::io;
use std::io::Write;

fn test(n:i64) -> i64 {
    let sum_sq: i64 = (1 as i64..n+1).fold(0, |acc, v| acc + v*v );
    let sum: i64 = (1 as i64..n+1).sum();
    sum*sum - sum_sq
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut s_t = String::new();
    io::stdin().read_line(&mut s_t)?;
    let input_t : i32 = s_t.trim().parse()?;
    for _ in 0..input_t {
        let mut s_n = String::new();
        io::stdin().read_line(&mut s_n)?;
        let n : i64 = s_n.trim().parse()?;
        writeln!(io::stdout(), "{}", test(n))?;
    }
    Ok(())
}
