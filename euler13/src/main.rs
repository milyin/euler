extern crate num;

use std::io;
use std::io::Write;
use num::{Integer, CheckedMul, CheckedAdd, Unsigned, ToPrimitive};
use std::str::FromStr;
use std::num::ParseIntError;
use std::ops::{MulAssign, Add};
use std::fmt;
use std::fmt::Display;
use std::iter;
use std::char::from_digit;
use std::fmt::Debug;

trait LongNumValue: Unsigned + Integer + MulAssign + CheckedMul + CheckedAdd + Copy + From<u8> + Display + ToPrimitive + Debug {}

impl<T> LongNumValue for T
    where T: Unsigned + Integer + MulAssign + CheckedMul + CheckedAdd + Copy + From<u8> + Display + ToPrimitive + Debug {}

#[derive(Debug)]
struct LongNum<T: LongNumValue>
{
    values: Vec<T>,
    powers: Vec<u8>,
    radix: u8,
}

impl<T: LongNumValue> LongNum<T>
{
    fn new(radix: u8) -> Self {
        Self {
            values: vec![T::zero()],
            powers: Vec::new(),
            radix: radix
        }
    }

    fn from_iter<IT: Iterator<Item = u8>>(iter: IT, radix: u8) -> Option<Self> {
        let mut res = Self {
            values: Vec::new(),
            powers: Vec::new(),
            radix: radix
        };
        let mut acc = T::zero();
        let mut mul = T::one();
        let mut power: u8 = 0;
        for d in iter {
            if d >= radix {
                return None;
            }
            let mut consumed = false;
            if let Some(inc) = mul.checked_mul(&d.into()) {
                if let Some(new_acc) = acc.checked_add(&inc) {
                    acc = new_acc;
                    power += 1;
                    consumed = true;
                    if let Some(m) = mul.checked_mul(&radix.into()) {
                        mul = m;
                        continue;
                    }
                }
            }
            res.values.push(acc);
            res.powers.push(power);
            if consumed {
                power = 0;
                mul = T::one();
                acc = T::zero();
            } else {
                power = 1;
                mul = res.radix.into();
                acc = d.into();
            }
        }
        if acc == T::zero() {
            res.powers.pop();
        } else {
            res.values.push(acc);
        }
        Some(res)
    }

    fn from_str_radix<S: AsRef<str>>(s: S, radix: u8) -> Result<Self, ParseIntError> {
        // TODO: there should be better way. At least move it to statics
        let error_empty = "".parse::<i32>().expect_err("get empty input error");
        let error_invalid_digit = "Z".parse::<i32>().expect_err("get invalid digit error");

        if s.as_ref().is_empty() {
            return Err(error_empty);
        }

        let mut digits = Vec::new();
        for c in s.as_ref().chars().rev() {
            if let Some(d) = c.to_digit(radix as u32) {
               digits.push(d as u8);
            } else {
                return Err(error_invalid_digit);
            }
        }
        let res = Self::from_iter(digits.into_iter(), radix).unwrap();
        Ok(res)
    }

    fn rev_digits(&self) -> LongNumDigits<T> {
       LongNumDigits {
           num: &self,
           pow: 0,
           pos: 0,
           zero: false
       }
    }
}

impl<T: LongNumValue> FromStr for LongNum<T> {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return LongNum::from_str_radix(s, 10);
    }
}

#[derive(Debug)]
struct LongNumDigits<'a, T: LongNumValue> {
    num: &'a LongNum<T>,
    pos: usize,
    pow: u8,
    zero: bool
}

impl<'a, T: LongNumValue> Iterator for LongNumDigits<'a, T> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.num.values.len() {
            return None;
        }
        debug_assert!(self.num.values.len() == self.num.powers.len()+1);
        if self.pos < self.num.powers.len() {
            if self.pow == self.num.powers[self.pos] {
                self.pow = 0;
                self.pos += 1;
            }
        } else if self.zero {
            return None
        }
        let v = self.num.values[self.pos];
        let radix : T = self.num.radix.into();
        let r = v / num::pow(radix, self.pow as usize);
        let d = r % self.num.radix.into();
        self.zero = r == d;
        debug_assert!(d.to_u8().is_some());
        self.pow += 1;
        d.to_u8()
    }
}

impl<T: LongNumValue> fmt::Display for LongNum<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for d in self.rev_digits().collect::<Vec<u8>>().into_iter().rev() {
           let c = from_digit(d as u32, self.radix as u32).unwrap();
           write!(f, "{}", c)?
        }
        Ok(())
    }
}


impl<T: LongNumValue> Add for LongNum<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let my_digits = self.rev_digits()
            .map(Some)
            .chain(iter::repeat(None));
        let other_digits = other.rev_digits()
            .map(Some)
            .chain(iter::repeat(None));
        let mut acc = 0 as u8;
        let mut digits = Vec::new();
        for (a,b) in my_digits.zip(other_digits) {
            match (a,b) {
                (None,None) => {
                    digits.push(acc);
                    break;
                }
                (Some(d), None) | (None, Some(d)) => {
                    acc += d;
                    digits.push(acc % self.radix);
                    acc /= self.radix;
                    debug_assert!(acc == 0 || acc == 1);
                }
                (Some(a), Some(b)) => {
                    acc += a + b;
                    digits.push(acc % self.radix);
                    acc /= self.radix;
                    debug_assert!(acc == 0 || acc == 1);
                }
            }
        }
        Self::from_iter(digits.into_iter(), self.radix).unwrap()
    }
}

fn test(vn50 : &Vec<String>) -> Result<String, ParseIntError> {
    let mut sum = LongNum::<u16>::new(10);
    for s in vn50 {
        let ln : LongNum<u16> = s.trim().parse()?;
        sum = sum + ln;
    }
    Ok(sum.to_string().chars().take(10).collect())
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut s_n = String::new();
    io::stdin().read_line(&mut s_n)?;
    let n : i32 = s_n.trim().parse()?;
    let mut vn50 = Vec::new();
    for _ in 0..n {
        let mut sn50 = String::new();
        io::stdin().read_line(&mut sn50)?;
        vn50.push(sn50);
    }
    writeln!(io::stdout(), "{}", test(&vn50)?)?;
    Ok(())
}
