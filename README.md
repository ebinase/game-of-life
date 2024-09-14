# ライフゲーム
シンプルな生命シミュレーションであるライフゲームのRust実装

## モード
### Basic
オリジナルのルールに基づいて実装したモード
https://ja.wikipedia.org/wiki/%E3%83%A9%E3%82%A4%E3%83%95%E3%82%B2%E3%83%BC%E3%83%A0

<img src="https://github.com/user-attachments/assets/511cfdd4-6e76-4cbb-869f-4d15fa3f39a5" width="40%" alt="ベーシックモードのライフゲーム">

### Advanced
オリジナルルールに資源の概念とランダム性を追加したモード

集団形成と移動が生存に有利に働く

<img src="https://github.com/user-attachments/assets/8e0b3357-d759-4f94-9a8b-c6f6eae72013" width="40%" alt="拡張モードのライフゲーム">



## 実行方法
```
$ cd /path/to/repo
$ cargo run -- -w 50 -h 25 -d 0.2 -m advanced
```

### オプション
```
$ cargo run -- --help
Usage: game-of-life [OPTIONS]
Options:
  -m, --mode <MODE>        ゲームモード（BASICまたはADVANCED） [default: basic]
  -w, --width <WIDTH>      セルを配置するフィールドの幅 [default: 20]
  -h, --height <HEIGHT>    セルを配置するフィールドの高さ [default: 10]
  -d, --density <DENSITY>  初期状態で何%の確率でセルを誕生させるか(0.0: 全滅 ~  1.0: 全て生存) [default: 0.2]
      --help

```


## 参考
- Rust
  - https://doc.rust-jp.rs/book-ja/title-page.html
  - https://www.tohoho-web.com/ex/rust.html
  - module
    - https://zenn.dev/hakoten/articles/058a681ba6fe4a
    - https://qiita.com/TakedaTakumi/items/7936a19979e46fc1b780
  - trait
    - https://zenn.dev/hakoten/articles/058a681ba6fe4a
    - https://zenn.dev/skanehira/scraps/66f0920c804b83
    - https://zenn.dev/mosu/articles/87e4715c4bcbb8
- CLI
  - clap, console
    - https://qiita.com/namn1125/items/5eb2c7cfecbf8870abe0
    - https://zenn.dev/kumavale/articles/f06fa100e394b5
  - strum
    - https://zenn.dev/fraim/articles/6b13df416f5544
- ライフゲーム
  - https://ja.wikipedia.org/wiki/%E3%83%A9%E3%82%A4%E3%83%95%E3%82%B2%E3%83%BC%E3%83%A0
  - https://hirlab.net/nblog/category/programming/art_616/
  - https://programing.style/archives/968  
