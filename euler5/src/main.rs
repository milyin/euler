use std::io;
use std::io::Write;

fn smallest_multiple(mut ns: Vec<i64>) -> i64 {
    let mut multiple: i64 = 1;
    for m in 2 as i64..{
        loop {
            let mut done = true;
            let mut b = false;
            for n in &mut ns {
                if *n != 1 {
                    done = false;
                }
                if *n % m == 0 {
                    *n /= m;
                    b = true;
                }
            }
            if done {
                return multiple;
            }
            if b {
                multiple *= m;
            } else {
                break;
            }
        }
    }
    panic!("unexpected end");
}

fn test(n:i64) -> i64 {
    let vec: Vec<i64> = (2..n+1).collect();
    smallest_multiple(vec)
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
