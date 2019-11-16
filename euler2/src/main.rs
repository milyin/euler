use std::io;
use std::io::Write;

struct Fibonacci {
    curr: i64,
    next: i64
}

impl Iterator for Fibonacci {
   type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {curr:1, next:1}
}

fn test(n : i64) -> i64 {
    fibonacci()
        .take_while(|v| v < &n)
        .filter(|v| v % 2 == 0 )
        .sum()
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
