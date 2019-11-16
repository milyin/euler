use std::io;
use std::io::Write;
use std::collections::HashMap;

type Lengths = HashMap<usize,u32>;

fn collatz(n: usize) -> usize {
    if n % 2 == 0 { n / 2 } else { 3 * n + 1 }
}

fn scan(mut n: usize, lengths: &mut Lengths) -> u32 {
    let mut len = 0;
    let mut stack = Vec::new();
    loop {
        if lengths.contains_key(&n) {
            len += lengths[&n];
            break;
        } else {
            stack.push(n);
            len += 1;
        }
        if n == 1 {
            break;
        }
        n = collatz(n);
    }
//    while let Some(pos) = stack.pop() {
//        lengths.insert(pos, len - stack.len() as u32);
//    }
    len
}

fn test(n: usize, lengths: &mut Lengths, maxes: &mut Vec<(u32,usize)>) -> usize {
    if n < maxes.len() {
        return maxes[n].1;
    }
    let last = if maxes.is_empty() {1} else { maxes.len() - 1 };
    let mut max = if maxes.is_empty() {(0,0)} else { maxes[last] };
    maxes.resize(n+1, (0,0));
    for m in last+1..=n {
        let len = scan(m, lengths);
        if len >= max.0 {
            max = (len, m);
        }
        maxes[m] = max;
    }
    max.1
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut maxes = Vec::new();
    let mut lengths = Lengths::new();
    let mut s_t = String::new();
    io::stdin().read_line(&mut s_t)?;
    let t : u32 = s_t.trim().parse()?;
    for _ in 0..t {
        let mut s_n = String::new();
        io::stdin().read_line(&mut s_n)?;
        let n : usize = s_n.trim().parse()?;
        writeln!(io::stdout(), "{}", test(n, &mut lengths, &mut maxes))?;
    }
    Ok(())
}
