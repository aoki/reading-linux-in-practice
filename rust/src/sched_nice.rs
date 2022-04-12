use nix::{
    libc::EXIT_FAILURE,
    sys::time::TimeSpec,
    time::{clock_gettime, ClockId},
    unistd::Pid,
};
use std::{env, process::exit};

const NLOOP_FOR_ESTIMATION: usize = 1_000_000_000;
const NSECS_PER_MSEC: usize = 1_000_000;
const NSECS_PER_SEC: usize = 1_000_000_000;

fn get_time() -> TimeSpec {
    match clock_gettime(ClockId::CLOCK_MONOTONIC) {
        Ok(time) => time,
        Err(e) => {
            eprintln!("clock_gettime() failed {:?}", e);
            std::process::exit(EXIT_FAILURE);
        }
    }
}

// 2つの TimeSpec の差分を計算しナノ秒で返却します
fn diff_nsec(before: TimeSpec, after: TimeSpec) -> usize {
    (after.tv_sec() as usize * NSECS_PER_SEC + after.tv_nsec() as usize)
        - (before.tv_sec() as usize * NSECS_PER_SEC + before.tv_nsec() as usize)
}

/// CPU時間を 1ms 使う処理に必要なループ回数を推定します
fn loops_per_msec() -> usize {
    let before = get_time();

    // リリースビルドしてしまうと最適化がかかり機能しなくなる
    for _ in 0..NLOOP_FOR_ESTIMATION {}

    let after = get_time();

    // ループ回数をかかった時間(diff_nsec) でわり、1nsあたりのループ回数を計算し、NSECS_PER_MSECを掛け、単位をmsにする
    NLOOP_FOR_ESTIMATION * NSECS_PER_MSEC / diff_nsec(before, after)
}

/// 与えられた引数を `usize` に変換します。
/// 変換に失敗した場合は、エラーを表示し終了します。
fn arg_validation(arg: &String, argname: &str) -> usize {
    match arg.parse::<usize>() {
        Ok(r) => {
            if r < 1 {
                eprintln!("<{}>({}) should be >= 1", argname, arg);
                std::process::exit(EXIT_FAILURE);
            }
            r
        }
        Err(e) => {
            eprintln!("<{}>({}) should be number: {:?}", argname, arg, e);
            std::process::exit(EXIT_FAILURE);
        }
    }
}

/// ## コマンドライン引数
/// - 第1引数（total）: プログラムを動作させる合計時間（ms単位）
/// - 第2引数（resol）: 統計情報の採取間隔（ms単位）
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("usage: {} <total[ms]> <resolution[ms]>", args[0]);
        exit(EXIT_FAILURE);
    }

    // 並行プロセス数
    let nproc = 2;
    // プログラムを動作させる合計時間(ms)
    let total = arg_validation(&args[1], "total");
    // 統計情報の採取時間(ms)
    let resol = arg_validation(&args[2], "resol");

    // プログラムを動作させる合計時間が統計情報の採取時間(resol: 解像度)で割り切れるかを確認
    if total % resol != 0 {
        eprintln!(
            "<total>({}) should be multiple of <resolution>({})",
            total, resol
        );
    }
    // 計測するレコード数を計算する
    // 100ms を 10ms 単位で計測する場合は、10 レコードとなる
    let nrecord = total / resol;

    // 1ms にかかるループ回数を計測し、それを解像度(ms)に合わせる
    let nloop_per_resol = loops_per_msec() * resol;

    let mut logbuf: Vec<TimeSpec> = Vec::<TimeSpec>::with_capacity(nrecord);
    let mut pids = Vec::<Pid>::with_capacity(nproc);

    let start = get_time();
    todo!();
}
