extern crate pollster;

use factorization::run;

fn main() {
    pollster::block_on(run())
}
