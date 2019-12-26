
extern crate clap;
extern crate num_bigint;
extern crate num_traits;

// Time Start: Tue, 24 Dec 2019 05:49:55 -0500
// Time Finish 1: Tue, 24 Dec 2019 07:29:32 -0500 (1 hour, 39 minutes, 37 seconds)
// Time Finish 2:
// Time Total:

use std::convert::TryFrom;
use std::fs;

use clap::{Arg, App};
use num_bigint::BigInt;
use num_traits::Signed;
use num_traits::ToPrimitive;

type Deck = Vec<u16>;


// Linear modular transformation
#[derive(Copy, Clone, Debug)]
struct LinModXForm {
    pub m: i64,
    pub b: i64,
    pub modulus: i64,
}
impl LinModXForm {
    pub fn new(m: i64, b: i64, mo: i64) -> LinModXForm {
        LinModXForm { m: m, b: b, modulus: mo }
    }

    fn bi_to_64(&self, bi: &BigInt, msg: &str) -> i64 {
        let mut bi = bi.clone() % self.modulus;
        if bi.is_negative() { bi = bi + self.modulus }
        // if bi.is_negative() { panic!("Modulus failure!?"); }
        // if bi > BigInt::from(self.modulus) { panic!("Modulus failure!?"); }
        i64::try_from(bi.to_u64().unwrap_or_else(|| panic!("{}", msg))).unwrap_or_else(|err| panic!("{}: {}", msg, err))
    }

    // Return func = self(g(x))
    pub fn compose(&self, g: LinModXForm) -> LinModXForm {
        assert_eq!(self.modulus, g.modulus);
        let m2 = BigInt::from(self.m);
        let m = m2.clone() * g.m;
        let b = m2 * g.b + self.b;
        return LinModXForm::new(self.bi_to_64(&m, "m too big!?"), self.bi_to_64(&b, "b too big!?"), self.modulus);
    }

    pub fn eval(&self, x: i64) -> i64 {
        let rv = BigInt::from(self.m) * x + self.b;
        return self.bi_to_64(&rv, "rv too big!?");
    }

    pub fn invert(&self, y: i64) -> i64 {
        fn inverse(a: i64, b: i64) -> i64 { // inverse of a, mod b
            let (mut r, mut r_, mut t, mut t_) = (b, a, 0, 1);
            while r_ != 0 {
                let q = r / r_;
                let tmp = t; t = t_; t_ = tmp - q * t_;
                let tmp = r; r = r_; r_ = tmp - q * r_;
            }
            // GCD(a,b) == r;
            assert_eq!(r, 1, "Expected coprime values");
            if t < 0 { t += b }
            return t;
        }

        let rv = (BigInt::from(y) - self.b) * inverse(self.m, self.modulus);
        return self.bi_to_64(&rv, "rv not a u64?");
    }

    pub fn pow(self, rhs: u64) -> LinModXForm {
        let bpow = BigInt::from(rhs);
        let bmod = BigInt::from(self.modulus);
        let m = BigInt::from(self.m).modpow(&bpow, &bmod);
        // new b = m(m(m(...)+1)+1) * b = (m^(n-1) + m^(m-2) + ... + 1) * b
        let b: BigInt;
        if self.m == 1 { b = bpow.clone() * self.b; }
        else { b = ((m.clone() - 1) / (self.m - 1)) * self.b; }
        return LinModXForm::new(self.bi_to_64(&m, "m too big!?"), self.bi_to_64(&b, "b too big!?"), self.modulus);
    }
}


#[derive(Copy, Clone, Debug)]
enum Technique {
    DealNew,
    Cut(i64),
    DealInc(i64),
}
use Technique::*;

impl Technique {
    pub fn apply(&self, deck: &mut Deck) {
        match self {
            DealNew => deck.reverse(),
            Cut(n) if *n >= 0 => deck.rotate_left(*n as usize),
            Cut(n) if *n < 0  => deck.rotate_right((-*n) as usize),
            DealInc(n) => {
                let tmp = deck.clone();
                let len = tmp.len();
                let mut i = 0;
                for val in tmp {
                    deck[i] = val;
                    i += *n as usize;
                    i %= len;
                }
            },
            _ => unreachable!("Stupid rust"),
        }
    }

    pub fn as_tx(&self, mo: i64) -> LinModXForm {
        match self {
            DealNew    => LinModXForm::new(mo-1, mo-1, mo),
            Cut(n)     => LinModXForm::new(1, mo-(*n), mo),
            DealInc(n) => LinModXForm::new(*n, 0, mo),
        }
    }
}


fn deck(n: u16) -> Deck { (0..n).collect() }

fn num_at(s: &str, n: usize) -> i64 {
    let mut ok = false;
    let mut sign = 1_i64;
    let mut rv = 0_i64;
    for ch in s.chars().skip(n) {
        match ch {
            '+' => (),
            '-' => sign *= -1,
            '0'..='9' => { ok = true; rv *= 10; rv += ch.to_digit(10).unwrap_or_else(|| unreachable!("Bummer")) as i64; },
             _  => break,
        }
    }
    if !ok { panic!("No number found at position {} in '{}'", n, s); }
    return sign * rv;
}


fn load_shuffle(fname: &String) -> Vec<Technique> {
    let contents = fs::read_to_string(fname)
        .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));

    let mut i = 0;
    return contents.lines().map(|l| {
        i += 1;
        if l == "deal into new stack" {
            DealNew
        } else if l.contains("increment") {
            DealInc(num_at(l, 20))
        } else if l.contains("cut") {
            Cut(num_at(l, 4))
        } else {
            panic!("Unexpected content '{}' on line {}", l, i);
        }
    }).collect();
}

fn main() {
    let matches = App::new("Advent of Code 2019, Day 22")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("22.in"));

    let shuffle = load_shuffle(&fname);
    let mut cards = deck(10007);
    let mut xform1 = LinModXForm::new(1, 0, 10007);
    let mut xform2 = LinModXForm::new(1, 0, 119315717514047);
    for tq in shuffle.iter() {
        tq.apply(&mut cards);
        xform1 = tq.as_tx(xform1.modulus).compose(xform1);
        xform2 = tq.as_tx(xform2.modulus).compose(xform2);
    }

    if let Some(idx) = cards.iter().position(|&x| x == 2019) {
        assert_eq!(idx, 6831);
        assert_eq!(xform1.eval(2019), 6831);
        println!("Step 1: 2019 is at index {} == {}", idx, xform1.eval(2019));
    } else {
        println!("Short test: {:?}", cards);
    }

    xform2 = xform2.pow(101741582076661);
    println!("Step 2: 2020 is at index {}", xform2.eval(2020));
    assert_eq!(xform2.invert(2020), 81781678911487);
    println!("Step 2: {} is at index 2020", xform2.invert(2020));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_new() {
        let mut cards = deck(10);
        DealNew.apply(&mut cards);
        assert_eq!(cards, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
        assert_eq!(DealNew.as_tx(10).eval(7), 2);
    }

    #[test]
    fn deal_inc() {
        let mut cards = deck(10);
        DealInc(3).apply(&mut cards);
        assert_eq!(cards, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
        assert_eq!(DealInc(3).as_tx(10).eval(7), 1);
    }

    #[test]
    fn cut() {
        let mut cards = deck(10);
        Cut(3).apply(&mut cards);
        assert_eq!(cards, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
        assert_eq!(Cut(3).as_tx(10).eval(7), 4);
        assert_eq!(Cut(3).as_tx(10).eval(2), 9);
        let mut cards = deck(10);
        Cut(-4).apply(&mut cards);
        assert_eq!(cards, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
        assert_eq!(Cut(-4).as_tx(10).eval(7), 1);
        assert_eq!(Cut(-4).as_tx(10).eval(2), 6);
    }

    fn test_seq(seq: &Vec<Technique>, n: u16, iter: usize) {
        let mut cards = deck(n);
        for _ in 0..iter {
            for act in seq.iter() {
                act.apply(&mut cards);
            }
        }
        let mut xform = LinModXForm::new(1, 0, n as i64);
        for act in seq.iter() {
            xform = act.as_tx(n as i64).compose(xform);
        }
        let xform = xform.pow(iter as u64);

        for (i, v) in cards.iter().enumerate() {
            assert_eq!(xform.eval(*v as i64), i as i64);
            assert_eq!(xform.invert(i as i64), *v as i64);
        }
    }

    #[test]
    fn xform() {
        let seq = vec![ Cut(6), DealInc(7), DealNew ];
        test_seq(&seq, 10, 1);
        test_seq(&seq, 10, 100);
        let seq = vec![
            DealNew,
            Cut(-2),
            DealInc(7),
            Cut(8),
            Cut(-4),
            DealInc(7),
            Cut(3),
            DealInc(9),
            DealInc(3),
            Cut(-1),
        ];
        test_seq(&seq, 10, 1);
        test_seq(&seq, 10, 100);

        let shuffle = load_shuffle(&String::from("22.in"));
        test_seq(&shuffle, 10007, 2);
    }
}
