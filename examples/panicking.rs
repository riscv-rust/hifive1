#![no_std]

extern crate hifive;

fn three() {
    panic!()
}

fn two() {
    three();
}

fn one() {
    two();
}

fn main() {
    hifive::init(115_200);
    one();
}
