extern crate rand;
extern crate sdl2;
use std::env;

mod automata;
pub use crate::automata::*;
mod fps_utils;
pub use crate::fps_utils::fps_utils::*;
mod framebuffer;
pub use crate::framebuffer::*;
mod fast_type;
pub use crate::fast_type::*;
mod ownership;
pub use crate::ownership::*;
mod concurrency;
pub use crate::concurrency::*;
mod opengl_test;
use crate::opengl_test::*;
mod memory;
use crate::memory::*;

const AUTOMATA: &str = "--automata";
const FAST_TYPE: &str = "--fasttype";
const FRAMEBUFFER: &str = "--framebuffer";
const OWNERSHIP: &str = "--ownership";
const CONCURRENCY: &str = "--concurrency";
const OPENGL_TEST: &str = "--opengltest";
const MEMORY: &str = "--memory";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "OPTIONS:\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
            AUTOMATA, FAST_TYPE, FRAMEBUFFER, OWNERSHIP, CONCURRENCY, OPENGL_TEST,
        );
    } else if args.len() > 2 {
        println!("Too many arguments!");
    } else {
        let arg: &str = &args[1][..];
        match arg {
            AUTOMATA => start_automata(),
            FAST_TYPE => start_fast_type(),
            FRAMEBUFFER => start_framebuffer(),
            OWNERSHIP => start_ownership(),
            CONCURRENCY => start_concurrency(),
            OPENGL_TEST => start_opengl_test(),
            MEMORY => start_memory(),
            _ => println!("Unknown arguement!"),
        }
    }
}
