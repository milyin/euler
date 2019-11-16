use std::io;
use std::io::Write;

fn is_palindrome(n: i32) -> bool {
    (n % 10) == ((n / 100000) % 10)
    && (((n / 10) % 10) == ((n / 10000) % 10))
    && (((n / 100) % 10) == ((n / 1000) % 10))
}

fn is_product(n: i32) -> bool {
    for m in 100..999 {
        if n % m == 0 {
            let k = n / m;
            if k >= 100 && k <= 999 {
                return true;
            }
        }
    }
    false
}

fn test(n:i32) -> i32 {
    let mut v = n;
    while v >= 101101 {
        if is_palindrome(v) {
            if is_product(v) {
                return v;
            }
        }
        v -= 1;
    }
    panic!("101101 must pass");
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut s_t = String::new();
    io::stdin().read_line(&mut s_t)?;
    let input_t : i32 = s_t.trim().parse()?;
    for _ in 0..input_t {
        let mut s_n = String::new();
        io::stdin().read_line(&mut s_n)?;
        let n : i32 = s_n.trim().parse()?;
        writeln!(io::stdout(), "{}", test(n-1))?;
    }
    Ok(())
}
