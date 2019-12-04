
// Time Start: Wed, 04 Dec 2019 11:28:03 -0500
// Time Finish 1: Wed, 04 Dec 2019 11:44:10 -0500 (16 minutes, 7 seconds)
// Time Finish 2: Wed, 04 Dec 2019 11:58:44 -0500 (14 minutes, 34 seconds)
// Time Total: 30 minutes, 41 seconds

fn num_ok1(mut num: u32) -> bool {
    let mut last = 99;
    let mut dupped = false;

    while num > 0 {
        let this = num % 10;
        if this > last { return false; }
        if this == last { dupped = true; }
        last = this;
        num = num / 10;
    }

    return dupped;
}

fn num_ok2(mut num: u32) -> bool {
    let mut last = 99;
    let mut dupped = -1;
    let mut double = false;

    while num > 0 {
        let this = num % 10;
        if this > last { return false; }
        if !double {
            if this == last {
                dupped -= 1;
            } else {
                 if dupped == -2 { double = true; }
                dupped = -1;
            }
        }
        last = this;
        num = num / 10;
    }
    if dupped == -2 { double = true; }

    return double;
}

fn main() {
    let (a, b) = (206938, 679128);

    let mut count1 = 0;
    let mut count2 = 0;
    for pass in a..=b {
        if num_ok1(pass) { count1 += 1; }
        if num_ok2(pass) { count2 += 1; }
    }
    println!("Step 1: Found {} candidate passwords", count1);
    println!("Step 2: Found {} candidate passwords", count2);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples1() {
        assert_eq!(num_ok1(111123), true);
        assert_eq!(num_ok1(122345), true);
        assert_eq!(num_ok1(111111), true);// Example 1
        assert_eq!(num_ok1(223450), false);// Example 2
        assert_eq!(num_ok1(123789), false);// Example 3
    }

    #[test]
    fn samples2() {
        assert_eq!(num_ok2(111123), false);
        assert_eq!(num_ok2(112345), true);
        assert_eq!(num_ok2(122345), true);
        assert_eq!(num_ok2(112233), true);
        assert_eq!(num_ok2(123444), false);
        assert_eq!(num_ok2(111122), true);
        assert_eq!(num_ok2(111111), false);// Example 1
        assert_eq!(num_ok2(223450), false);// Example 2
        assert_eq!(num_ok2(123789), false);// Example 3
    }
}
