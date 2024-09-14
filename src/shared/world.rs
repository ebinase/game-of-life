use std::fmt::Display;

pub trait World: Display {
    fn new(width: usize, height: usize, density: f64) -> Self;
    fn update(&self) -> Self;
}
