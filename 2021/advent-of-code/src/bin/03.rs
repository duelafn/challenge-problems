// SPDX-License-Identifier: MIT

fn records(fname: &str) -> Vec<Vec<u32>> {
    let contents = std::fs::read_to_string(fname).unwrap();

    return contents.lines().enumerate().map(
        |(_, chunk)| {
            chunk.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
        }
    ).collect();
}

fn popcon<'a, I>(mut v: I) -> Vec<i32> where I: Iterator<Item=&'a Vec<u32>> {
    let frst = v.next().unwrap();
    let mut count: Vec<i32> = frst.iter().map(|v| 2*(*v as i32) - 1).collect();
    for row in v {
        for (i, v) in row.iter().enumerate() {
            match v {
                0 => count[i] -= 1,
                1 => count[i] += 1,
                d => panic!("unexpected digit: {}", d),
            }
        }
    }
    return count;
}

fn gamma(v: &Vec<Vec<u32>>) -> u32 {
    let count = popcon(v.iter());
    let mut rv = 0;
    for c in count.iter() {
        rv <<= 1;
        if *c >= 0 { rv += 1; }
    }
    return rv;
}

fn filterer<F>(v: &Vec<Vec<u32>>, chooser: F) -> u32 where F: Fn(i32) -> u32 {
    let mut idx = 0;
    let mut pop = popcon(v.iter());
    let mut keep = chooser(pop[idx]);
    let mut lst: Vec<&Vec<u32>> = v.iter().filter(|row| row[idx] == keep).collect();
    while lst.len() > 1 {
        idx += 1;
        pop = popcon(lst.iter().cloned());
        keep = chooser(pop[idx]);
        lst = lst.iter().cloned().filter(|row| row[idx] == keep).collect();
    }
    let mut rv = 0;
    for c in lst[0].iter() {
        rv <<= 1;
        if *c > 0 { rv += 1; }
    }
    return rv;
}

fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("03.in"));

    let data = records(&fname);
    let gamma = gamma(&data);
    let epsilon = gamma ^ ((1 << data[0].len()) - 1);
    println!("part 1: {} * {} = {}", gamma, epsilon, gamma * epsilon);

    let oxygen = filterer(&data, |ct| if ct >= 0 { 1 } else { 0 });
    let co2    = filterer(&data, |ct| if ct <  0 { 1 } else { 0 });
    println!("part 2: {} * {} = {}", oxygen, co2, oxygen * co2);
}
