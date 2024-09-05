mod cell;
mod matrix;
mod world;

use crate::cell::{AliveContext, CellState, DeadContext};
use crate::world::World;
use std::thread::sleep;
use std::time::Duration;

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Generation: {}\n", self.gen)?;
        for line in self.cells.as_slice().chunks(self.width) {
            for &cell in line {
                let symbol = match cell {
                    CellState::Alive(context) => match context {
                        AliveContext::Birth => '〇',
                        AliveContext::Survive => '〇',
                    },
                    CellState::Dead(context) => match context {
                        DeadContext::Overpopulated => '・',
                        DeadContext::Underpopulated => '・',
                        DeadContext::CannotBirth => '・',
                    },
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    println!("Welcome to Game Of Life!");

    let mut world = World::new(40, 25, 0.3);
    println!("{}", world);
    sleep(Duration::from_secs(1));

    loop {
        world = world.update();
        println!("{}", world);
        sleep(Duration::from_secs(1));
    }
}
