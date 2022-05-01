//! # 測定内容
//! - I/O サイズによる性能の変化
//! - シーケンシャルアクセスとランダムアクセスの違い
//!
//! # プログラム仕様
//! - 指定したパーティションの先頭から1Gバイトまでの領域内に、合計64MバイトのI/Oを発行する
//! - 読み書きの種類、アクセスパターン（シーケンシャルアクセス、ランダムアクセス）、及び1回あたりのI/Oサイズを指定できる
//! - 受け取る引数
//!     - 第1引数: ファイル名
//!     - 第2引数: 本章の後半において説明する、カーネルによるI/O支援機能を有効にするかどうか（on, off）
//!     - 第3引数: 読み書きの種類（r = 読み出し、 w = 書き込み）
//!     - 第4引数: アクセスパターン（sec = シーケンシャルアクセス、 rand = ランダムアクセス）
//!     - 第5引数: 1回あたりのI/Oサイズ（Kバイト）

use nix::libc::{malloc, EXIT_FAILURE};
use std::{env, ffi::c_void};

const PART_SIZE: usize = 1024 * 1024 * 1024; // 1GB
const ACCESS_SIZE: usize = 64 * 1024 * 1024; // 64MB

fn main() {
    let argv: Vec<String> = env::args().collect();

    // 引数の数をチェック
    let progname = &argv[0];
    if argv.len() != 6 {
        eprintln!(
            "usage: {} <filename> <kernel's help> <r/w> <access pattern> <block size[KB]>",
            progname
        );
        std::process::exit(EXIT_FAILURE);
    }

    // ファイル名を取得
    let filename = &argv[1];

    // カーネルによるI/O支援機能フラグを取得
    let help = match argv[2].as_str() {
        "on" => true,
        "off" => false,
        _ => {
            eprintln!("kernel's help should be 'on' or 'off': {}", argv[2]);
            std::process::exit(EXIT_FAILURE);
        }
    };

    // 読み書きの種類を取得
    let write_flag = match argv[3].as_str() {
        "r" => false,
        "w" => true,
        _ => {
            eprintln!("r/w should be 'r' or 'w': {}", argv[3]);
            std::process::exit(EXIT_FAILURE);
        }
    };

    // ランダムアクセスかシーケンシャルアクセスかを取得
    let random = match argv[4].as_str() {
        "seq" => false,
        "rand" => true,
        _ => {
            eprintln!("access pattern should be 'seq' or 'rand: {}", argv[4]);
            std::process::exit(EXIT_FAILURE);
        }
    };

    // 1アクセスあたりのサイズを取得（KB）
    let block_size = match argv[5].parse::<usize>() {
        Ok(0) | Err(_) => {
            eprintln!("block size should be number and > 0: {}", argv[5]);
            std::process::exit(EXIT_FAILURE);
        }
        Ok(size) => size * 1024,
    };
    if ACCESS_SIZE % block_size != 0 {
        eprintln!(
            "access size({}) should be multiple of block size: {}",
            ACCESS_SIZE, argv[5]
        );
        std::process::exit(EXIT_FAILURE);
    }

    let max_count = PART_SIZE / block_size;
    let count = ACCESS_SIZE / block_size;

    let offset: *mut c_void;
    unsafe {
        offset = malloc(max_count * std::mem::size_of::<usize>());
    }
    if offset == std::ptr::null_mut() {
        eprintln!("malloc() failed");
        std::process::exit(EXIT_FAILURE);
    }

    // `O_DIRECT` フラグを与えることで、ダイレクトI/Oを使う
}
