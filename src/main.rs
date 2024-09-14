mod advanced;
mod basic;
mod shared;

use crate::advanced::world::AdvancedWorld;
use crate::basic::world::BasicWorld;
use crate::shared::world::World;
use clap::Parser;
use std::thread::sleep;
use std::time::Duration;

// See. https://docs.rs/strum/latest/strum/derive.EnumString.html
#[derive(Debug, Clone, strum::EnumString, strum::Display)]
#[strum(serialize_all = "lowercase")]
enum GameMode {
    Basic,
    Advanced,
}

#[derive(Parser, Debug)]
#[command(disable_help_flag = true)]
struct Args {
    /// ゲームモード（BASICまたはADVANCED）
    #[arg(short, long, default_value_t = GameMode::Basic, value_parser = clap::value_parser!(GameMode))]
    mode: GameMode,

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

    println!("ゲームモード: {}", args.mode);

    match args.mode {
        GameMode::Basic => execute(BasicWorld::new(args.width, args.height, args.density)),
        GameMode::Advanced => execute(AdvancedWorld::new(args.width, args.height, args.density)),
    }
}

fn execute(mut world: impl World) {
    println!("{}", world);
    sleep(Duration::from_secs(1));

    loop {
        world = world.update();
        println!("{}", world);
        sleep(Duration::from_millis(300));
    }
}
