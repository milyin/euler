use std::io;
use std::io::Write;
use std::collections::VecDeque;

fn test(s: &str, k : usize) -> i64 {
    let mut mul:i64 = 1;
    let mut maxmul:Option<i64> = None;
    let mut deque = VecDeque::with_capacity(k);
    let mut zeroes = 0;

    for c in s.chars() {
        let n : i64 = c.to_string().parse().expect("digit expected");
        if n != 0 {
            mul *= n;
        } else {
            zeroes += 1;
        }
        deque.push_back(n);
        if deque.len() == k {
            if zeroes == 0 && mul > maxmul.unwrap_or(0) {
                maxmul = Some(mul);
            }
            let n = deque.pop_front().expect("deque must be not empty");
            if n != 0 {
                mul /= n;
            } else {
                zeroes -= 1;
            }
        }
    }
    maxmul.unwrap_or(0)
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut s_t = String::new();
    io::stdin().read_line(&mut s_t)?;
    let input_t : i32 = s_t.trim().parse()?;
    for _ in 0..input_t {
        let mut s_nk = String::new();
        let mut s_v = String::new();
        io::stdin().read_line(&mut s_nk)?;
        io::stdin().read_line(&mut s_v)?;
        let mut it_nk = s_nk.split(' ');
        let _n : usize = it_nk.next().ok_or("N expected")?.trim().parse()?;
        let k : usize = it_nk.next().ok_or("K expected")?.trim().parse()?;
        writeln!(io::stdout(), "{}", test(&s_v.trim(), k))?;
    }
    Ok(())
}

