use nix::{
    libc::EXIT_FAILURE,
    time::{clock_gettime, ClockId},
};
use std::env;

const NLOOP_FOR_ESTIMATION: usize = 1_000_000_000;

fn loops_per_msec() {
    let before = clock_gettime(ClockId::CLOCK_MONOTONIC);
    // TODO: here
}

#[inline]
const fn arg_validation(arg: &String, argname: &str) -> usize {
    match arg.parse::<usize>() {
        Ok(a) => a,
        Err(_) => {
            eprintln!("<{}>({}) should be >= 1", argname, arg);
            std::process::exit(EXIT_FAILURE);
        }
    }
}

/// # コマンドライン引数
/// 第1引数（n）: 同時に動かすプロセス数
/// 第2引数（total）: プログラムを動作させる合計時間（ms単位）
/// 第3引数（resol）: 統計情報の採取間隔（ms単位）
fn main() {
    let ret: i32 = EXIT_SUCCESS;
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("usage {} <nproc> <total[ms]> <resolution[ms]>", args[0]);
        std::process::exit(EXIT_FAILURE);
    }

    let nproc = arg_validation(&args[1], "nproc");
    let total = arg_validation(&args[2], "total");
    let resol = arg_validation(&args[3], "resol");

    if total % resol != 0 {
        eprintln!(
            "<total>({}) should be multiple of <resolution>({})",
            total, resol
        );
    }

    let nrecord = total / resol;

    std::process::exit(ret);
}
