use std::io;
use std::io::Write;
use std::cmp::max;

const N: usize = 20;

fn test(g:[[i64;N];N]) -> i64 {
    let mut mp: i64 = 0;
    for r in 0..N {
        for c in 0..N {
            let cp: i64 = if r + 3 < N {
                g[r][c] * g[r + 1][c] * g[r + 2][c] * g[r + 3][c]
            } else {
                0
            };
            let rp: i64 = if c + 3 < N {
                g[r][c] * g[r][c + 1] * g[r][c + 2] * g[r][c + 3]
            } else {
                0
            };
            let dpr: i64 = if r + 3 < N && c + 3 < N {
                g[r][c] * g[r + 1][c + 1] * g[r + 2][c + 2] * g[r + 3][c + 3]
            } else {
                0
            };
            let dpl: i64 = if r + 3 < N && c >= 3 {
                g[r][c] * g[r + 1][c - 1] * g[r + 2][c - 2] * g[r + 3][c - 3]
            } else {
                0
            };
            mp = max(mp, max(cp, max( rp, max(dpr, dpl))));
        }
    }
   mp
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut grid = [[0 as i64; N]; N];
    for r in 0..N {
        let mut s_r = String::new();
        io::stdin().read_line(&mut s_r)?;
        let mut it_r = s_r.trim().split(' ');
        for c in 0..N {
            grid[r][c] = it_r.next().ok_or("value expected")?.parse()?;
        }
    }
    writeln!(io::stdout(), "{}", test(grid))?;
    Ok(())
}
