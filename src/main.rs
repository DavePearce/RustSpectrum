// Doesn't do much yet

mod z80;
mod zxspectrum;

use zxspectrum::memory::*;

fn main() {
    print!("Welcome to Rust Spectrum...");

    let ram = zxspectrum::memory::create();
}
