// SPDX-License-Identifier: MIT

const NULL: u8 = 255;

/// Packed into a list. For reduced values, the even indices will all be NULL.
/// See tests for example packings.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Value {
    vec: [u8; 32],
}

impl Value {
    fn new() -> Value { Value { vec: [NULL; 32] } }

    fn parse(line: &str) -> Value {
        let mut v = Value::new();
        let mut d = 0_u32;
        let mut i = 0_usize;
        for ch in line.chars() {
            match ch {
                '0'..='9' => { i += 2_usize.pow(5-d) - 1; v.vec[i] = ch.to_digit(10).unwrap() as u8; i += 1; },
                'A'..='F' => { i += 2_usize.pow(5-d) - 1; v.vec[i] = ch.to_digit(16).unwrap() as u8; i += 1; }, // just for tests
                '['       => { d += 1; },
                ']'       => { d -= 1; },
                _         => { },
            }
        }
        return v;
    }

    fn magnitude(&self) -> usize { self._magnitude(0, 32) }
    fn _magnitude(&self, a: usize, len: usize) -> usize {
        if self.vec[a+len/2-1] == NULL { return self.vec[a+len-1] as usize; }
        return 3 * self._magnitude(a, len/2) + 2 * self._magnitude(a+len/2, len/2);
    }

    fn reduce(&mut self) { while self.step() { } }

    fn step(&mut self) -> bool {
        for i in 0..16 {
            if self.vec[2*i] != NULL { self.explode(2*i); return true; }
        }
        for i in 0..16 {
            if self.vec[2*i+1] != NULL && self.vec[2*i+1] > 9 { self.split(2*i+1); return true; }
        }
        return false;
    }

    fn explode(&mut self, mut i: usize) {
        assert!(i % 2 == 0);
        assert!(i < 31);
        assert!(self.vec[i] != NULL);
        assert!(self.vec[i+1] != NULL);
        let (a, b) = (self.vec[i], self.vec[i+1]);
        self.vec[i] = NULL;
        self.vec[i+1] = 0;
        for r in i+2..32 { if self.vec[r] != NULL { self.vec[r] += b; break; } }
        while i > 0 { i -= 1; if self.vec[i] != NULL { self.vec[i] += a; return; } }
    }

    fn split(&mut self, i: usize) {
        assert!(i % 2 == 1);
        assert!(i < 32);
        assert!(i > 0);
        assert!(self.vec[i] != NULL);
        assert!(self.vec[i-1] == NULL);
        let a = self.vec[i] / 2;
        self.vec[i] -= a;
        let mut off = 1;
        while i > 2*off && self.vec[i-2*off] == NULL { off *= 2; }
        self.vec[i-off] = a;
    }
}

impl std::ops::Add<Value> for Value {
    type Output = Value;
    fn add(self, other: Value) -> Value { &self + &other }
}
impl std::ops::Add<&Value> for Value {
    type Output = Value;
    fn add(self, other: &Value) -> Value { &self + other }
}
impl std::ops::Add<&Value> for &Value {
    type Output = Value;
    fn add(self, other: &Value) -> Value {
        let mut rv = Value::new();
        for i in 0..16 {
            if self.vec[2*i] != NULL || other.vec[2*i] != NULL { panic!("Addition of unreduced values!"); }
            rv[i]    = self.vec[2*i+1];
            rv[i+16] = other.vec[2*i+1];
        }
        rv.reduce();
        return rv;
    }
}

impl std::ops::Deref for Value {
    type Target = [u8; 32];
    fn deref(&self) -> &Self::Target { &self.vec }
}
impl std::ops::DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.vec }
}


fn records(fname: &str) -> Vec<Value> {
    let contents = std::fs::read_to_string(fname).unwrap();
    contents.lines().map(|line| Value::parse(line)).collect()
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("18.in"));
    let lines = records(&fname);

    let mut iter = lines.iter();
    let mut sum = iter.next().unwrap().clone();
    for x in iter { sum = sum + x; }
    println!("Part 1: {}", sum.magnitude());

    let mut max = 0;
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i != j {
                let mag = (&lines[i] + &lines[j]).magnitude();
                if mag > max { max = mag; }
            }
        }
    }
    println!("Part 2: {}", max);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test18() {
        let mut x = Value::new();
        x[15] = 1; x[31] = 2;
        assert_eq!(&Value::parse("[1,2]"), &x);

        let mut x = Value::new();
        x[7] = 1; x[15] = 2; x[31] = 3;
        assert_eq!(&Value::parse("[[1,2],3]"), &x);

        let mut x = Value::new();
        x[15] = 9; x[23] = 8; x[31] = 7;
        assert_eq!(&Value::parse("[9,[8,7]]"), &x);

        let mut x = Value::new();
        x[7] = 1; x[15] = 9; x[23] = 8; x[31] = 5;
        assert_eq!(&Value::parse("[[1,9],[8,5]]"), &x);

        let x = Value { vec: [NULL,1,NULL,2,NULL,3,NULL,4,NULL,5,NULL,6,NULL,7,NULL,8,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,NULL,9] };
        assert_eq!(&Value::parse("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]"), &x);

        let x = Value { vec: [NULL,NULL,NULL,9,NULL,3,NULL,8,NULL,0,NULL,9,NULL,NULL,NULL,6,NULL,3,NULL,7,NULL,4,NULL,9,NULL,NULL,NULL,NULL,NULL,NULL,NULL,3] };
        assert_eq!(&Value::parse("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]"), &x);

        let x = Value { vec: [NULL,1,NULL,3,NULL,5,NULL,3,NULL,1,NULL,3,NULL,8,NULL,7,NULL,4,NULL,9,NULL,6,NULL,9,NULL,8,NULL,2,NULL,7,NULL,3] };
        assert_eq!(&Value::parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"), &x);

        println!("1");
        let mut x = Value::parse("[[[[[9,8],1],2],3],4]"); assert!(x.step()); assert_eq!(x, Value::parse("[[[[0,9],2],3],4]"));
        println!("2");
        let mut x = Value::parse("[7,[6,[5,[4,[3,2]]]]]"); assert!(x.step()); assert_eq!(x, Value::parse("[7,[6,[5,[7,0]]]]"));
        println!("3");
        let mut x = Value::parse("[[6,[5,[4,[3,2]]]],1]"); assert!(x.step()); assert_eq!(x, Value::parse("[[6,[5,[7,0]]],3]"));
        println!("4");
        let mut x = Value::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"); assert!(x.step()); assert_eq!(x, Value::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));
        println!("5");
        let mut x = Value::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"); assert!(x.step()); assert_eq!(x, Value::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
        println!("6");
        let mut x = Value::parse("[[[[0,7],4],[F,[0,D]]],[1,1]]"); assert!(x.step()); assert_eq!(x, Value::parse("[[[[0,7],4],[[7,8],[0,D]]],[1,1]]"));
        println!("7");
        let mut x = Value::parse("[[[[0,7],4],[[7,8],[0,D]]],[1,1]]"); assert!(x.step()); assert_eq!(x, Value::parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"));
        println!("8");
        let mut x = Value::parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"); assert!(x.step()); assert_eq!(x, Value::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
        println!("9");
        let mut x = Value::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"); assert!(!x.step()); assert_eq!(x, Value::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

        let a = Value::parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = Value::parse("[1,1]");
        assert_eq!(a+b, Value::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

        assert_eq!(Value::parse("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(Value::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(), 1384);
        assert_eq!(Value::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
        assert_eq!(Value::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
        assert_eq!(Value::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(), 1137);
        assert_eq!(Value::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(), 3488);
    }
}
