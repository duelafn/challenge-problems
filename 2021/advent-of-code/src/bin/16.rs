// SPDX-License-Identifier: MIT

fn num(vec: &Vec<u8>) -> i64 {
    let mut rv = 0;
    for b in vec {
        rv *= 2;
        rv += *b as i64;
    }
    return rv;
}

trait PacketStream: Iterator<Item=u8> + std::iter::ExactSizeIterator {
    fn read_bits(&mut self, n: usize) -> Vec<u8> {
        let mut rv = Vec::with_capacity(n);
        self.extend_bits(n, &mut rv);
        return rv;
    }

    fn read_i64(&mut self, n: usize) -> i64 { num(&self.read_bits(n)) }

    fn extend_bits(&mut self, n: usize, rv: &mut Vec<u8>) {
        for _ in 0..n { rv.push(self.next().unwrap()); }
    }
}

impl<I> PacketStream for I where I: Iterator<Item=u8> + std::iter::ExactSizeIterator { }



#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Packet {
    version: u8,
    type_id: u8,
    data: PacketData,
}

impl Packet {
    fn read_from<S: PacketStream>(stream: &mut S) -> Packet {
        let version = stream.read_i64(3) as u8;
        let type_id = stream.read_i64(3) as u8;
        let data = match type_id {
            4 => PacketData::read_literal_from(stream),
            t => PacketData::read_operator_from(stream, t),
        };
        return Packet { version, type_id, data };
    }

    fn checksum(&self) -> i64 {
        (self.version as i64) + self.data.checksum()
    }

    fn value(&self) -> i64 {
        self.data.value()
    }
}



use PacketData::*;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum PacketData {
    Literal(i64),
    Sum(Vec<Packet>),
    Prod(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Greater(Vec<Packet>),
    Less(Vec<Packet>),
    Equal(Vec<Packet>),
}

impl PacketData {
    fn read_literal_from<S: PacketStream>(stream: &mut S) -> PacketData {
        let mut rv = 0;
        loop {
            let prefix = stream.next().unwrap();
            rv = (rv << 4) + stream.read_i64(4);
            if prefix == 0 { break; }
        }
        return Literal(rv);
    }

    fn new_operator(t: u8, vec: Vec<Packet>) -> PacketData {
        match t {
            0 => Sum(vec),
            1 => Prod(vec),
            2 => Min(vec),
            3 => Max(vec),
            5 => Greater(vec),
            6 => Less(vec),
            7 => Equal(vec),
            t => panic!("Unexpected type {}", t),
        }
    }

    fn read_operator_from<S: PacketStream>(stream: &mut S, t: u8) -> PacketData {
        let typ = stream.next().unwrap();
        if typ == 0 {
            let n = stream.read_i64(15) as usize;
            let mut substream = stream.read_bits(n).into_iter();
            let mut vec = Vec::new();
            while substream.len() >= 6 { vec.push(Packet::read_from(&mut substream)); }
            return PacketData::new_operator(t, vec);
        }

        else if typ == 1 {
            let n = stream.read_i64(11) as usize;
            let mut vec = Vec::new();
            for _ in 0..n { vec.push(Packet::read_from(stream)); }
            return PacketData::new_operator(t, vec);
        }

        else { panic!("Bummer"); }
    }

    fn value(&self) -> i64 {
        match self {
            Literal(val) => *val,
            Sum(vec)     => vec.iter().map(|p| p.value()).sum(),
            Prod(vec)    => vec.iter().map(|p| p.value()).product(),
            Min(vec)     => vec.iter().map(|p| p.value()).min().unwrap(),
            Max(vec)     => vec.iter().map(|p| p.value()).max().unwrap(),
            Greater(vec) => if vec[0].value()  > vec[1].value() { 1 } else { 0 },
            Less(vec)    => if vec[0].value()  < vec[1].value() { 1 } else { 0 },
            Equal(vec)   => if vec[0].value() == vec[1].value() { 1 } else { 0 },
        }
    }

    fn checksum(&self) -> i64 {
        match self {
            Literal(_) => 0,
            Sum(vec) | Prod(vec) | Min(vec) | Max(vec) | Greater(vec) | Less(vec) | Equal(vec) =>
                vec.iter().map(|p| p.checksum()).sum(),
        }
    }
}


fn load(fname: &str) -> Vec<u8> {
    let contents = std::fs::read_to_string(fname).unwrap();
    let mut rv = Vec::with_capacity(4*contents.trim().len());
    for ch in contents.trim().chars() {
        let d = ch.to_digit(16).unwrap();
        if d > 15 { panic!("Invalid input"); }
        rv.push(if 0 == d & 0b1000 { 0 } else { 1 });
        rv.push(if 0 == d & 0b0100 { 0 } else { 1 });
        rv.push(if 0 == d & 0b0010 { 0 } else { 1 });
        rv.push(if 0 == d & 0b0001 { 0 } else { 1 });
    }
    return rv;
}


fn main() {
    let fname = std::env::args().skip(1).next().unwrap_or(String::from("16.in"));
    let input = load(&fname);
    let mut stream  = input.iter().copied();

    let packet = Packet::read_from(&mut stream);
    println!("Part 1: {}", packet.checksum());
    println!("Part 2: {}", packet.value());
}
