
extern crate clap;

// Time Start: Mon, 09 Dec 2019 13:31:55 -0500
// Time Finish 1:
// Time Finish 2:
// Time Total:

use std::fs;
use std::fmt;

use clap::{Arg, App};

type ImageWord = u8;


#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct LayerIterator<'a> {
    img: &'a Image,
    a: usize,
    b: usize,
}
impl<'a> LayerIterator<'a> {
    pub fn new(img: &'a Image) -> LayerIterator {
        LayerIterator { img: img, a: 0, b: img.num_layers() - 1 }
    }
}
impl<'a> Iterator for LayerIterator<'a> {
    type Item = &'a [ImageWord];
    #[inline]
    fn next(&mut self) -> Option<&'a [ImageWord]> {
        let i = self.a;
        if i > self.b { return None }
        self.a += 1;
        Some(self.img.layer_peek(i))
    }
}
impl<'a> DoubleEndedIterator for LayerIterator<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a [ImageWord]> {
        let i = self.b;
        if i < self.a { return None }
        self.b -= 1;
        Some(self.img.layer_peek(i))
    }
}
impl<'a> ExactSizeIterator for LayerIterator<'a> {
    fn len(&self) -> usize { self.img.num_layers() }
}


pub struct Image {
    width: usize,
    height: usize,
    data: Vec<ImageWord>,
}
impl Image {
    pub fn new(w: usize, h: usize) -> Image {
        return Image { width: w, height: h, data: Vec::new() }
    }
    pub fn load(fname: &String, w: usize, h: usize) -> Image {
        let contents = fs::read_to_string(fname)
            .unwrap_or_else(|err| panic!("Error reading {}: {}", fname, err));
        let data = contents.trim().chars().map(|c| c.to_digit(10).unwrap_or_else(|| panic!("Not a digit '{}'", c)) as ImageWord).collect::<Vec<ImageWord>>();
        if 0 != data.len() % (w * h) {
            panic!("Did not read a whole number of layers");
        }
        return Image { width: w, height: h, data: data }
    }

    pub fn num_layers(&self) -> usize {
        self.data.len() / (self.width * self.height)
    }

    pub fn flatten(&self) -> Vec<ImageWord> {
        let mut flat: Vec<ImageWord> = Vec::with_capacity(self.layer_len());
        for (l, layer) in self.layers().enumerate() {
            for i in 0..self.layer_len() {
                if l == 0 {
                    flat.push(layer[i]);
                } else if flat[i] == 2 {
                    flat[i] = layer[i];
                }
            }
        }
        return flat;
    }

    pub fn format_layer(&self, layer: &Vec<ImageWord>) -> String {
        layer.chunks_exact(self.width).map(|row| {
            row.iter().map(|data| match data {
                0 => '.', // black
                1 => '#', // white
                2 => ' ', // transparent
                _ => panic!("Invalid ImageWord value '{}'", data)
            }).collect::<String>()
        }).collect::<Vec<String>>().join("\n")
    }

    #[inline]
    pub fn layer_len(&self) -> usize { self.width * self.height }
    pub fn layer_peek(&self, i: usize) -> &[ImageWord] { &self.data[(i * self.layer_len())..((i+1) * self.layer_len())] }
    pub fn layers(&self) -> LayerIterator { LayerIterator::new(&self) }
}
impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_layer(&self.flatten()))
    }
}



fn main() {
    let matches = App::new("Advent of Code 2019, Day 08")
        .arg(Arg::with_name("FILE")
             .help("Input file to process")
             .index(1))
        .get_matches();
    let fname = String::from(matches.value_of("FILE").unwrap_or("08.in"));

    let img = Image::load(&fname, 25, 6);

    let mut zeroes = std::usize::MAX;
    let mut score = 0;
    let mut min_layer = std::usize::MAX;
    for (i, layer) in img.layers().enumerate() {
        let z = layer.iter().filter(|&n| *n == 0).count();
        if z < zeroes {
            let ones = layer.iter().filter(|&n| *n == 1).count();
            let twos = layer.iter().filter(|&n| *n == 2).count();
            zeroes = z;
            score = ones * twos;
            min_layer = i;
        }
    }

    println!("Part 1: Layer {} has {} zeroes and score {}", min_layer, zeroes, score);

    println!("Part 2:\n{}", img);
}
