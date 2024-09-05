mod cell;
mod matrix;
mod world;

use crate::cell::{AliveContext, CellState, DeadContext};
use crate::world::World;
use std::thread::sleep;
use std::time::Duration;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(disable_help_flag = true)]
struct Args {
    /// セルを配置するフィールドの幅
    #[arg(short, long, default_value_t = 20)]
    width: usize,

    /// セルを配置するフィールドの高さ
    #[arg(short, long, default_value_t = 10)]
    height: usize,

    /// 初期状態で何%の確率でセルを誕生させるか(0.0: 全滅 ~  1.0: 全て生存)
    #[arg(short, long, default_value_t = 0.3)]
    density: f64,
}

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

    let args = Args::parse();

    let mut world = World::new(args.width, args.height, args.density);
    println!("{}", world);
    sleep(Duration::from_secs(1));

    loop {
        world = world.update();
        println!("{}", world);
        sleep(Duration::from_secs(1));
    }
}
