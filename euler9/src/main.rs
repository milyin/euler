use std::io;
use std::io::Write;

fn test(n: i64) -> i64 {
    let mut abc = -1;
    for b in 1..n-1 {
        let c2nmb = n*n+2*b*b-2*n*b;
        if c2nmb % (2*(n-b)) != 0 {
            continue;
        }
        let c = c2nmb / (2*(n-b));
        let a = n-b-c;
        if a < 1 {
            continue;
        }
        if a*a + b*b == c*c {
            let abc_new = a*b*c;
            if abc_new > abc {
                abc = abc_new;
            }
        }
    }
    abc
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut s_t = String::new();
    io::stdin().read_line(&mut s_t)?;
    let input_t : i32 = s_t.trim().parse()?;
    for _ in 0..input_t {
        let mut s_n = String::new();
        io::stdin().read_line(&mut s_n)?;
        let n: i64 = s_n.trim().parse()?;
        writeln!(io::stdout(), "{}", test(n))?;
    }
    Ok(())
}
