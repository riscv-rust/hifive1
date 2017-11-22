#![no_std]

extern crate hifive;

fn three(_1: u32, _2: u32) {
    panic!()
}

fn two() {
    three(0x0123_4567, 0x1234_5678);
}

fn one() {
    two();
}

fn main() {
    one();
}
