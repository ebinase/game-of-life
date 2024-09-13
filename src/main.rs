mod advanced;
mod shared;
mod basic;

use clap::Parser;
use std::thread::sleep;
use std::time::Duration;
use crate::basic::world::World;

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
    #[arg(short, long, default_value_t = 0.2)]
    density: f64,
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
        sleep(Duration::from_millis(300));
    }
}
