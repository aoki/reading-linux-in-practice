use nix::{
    libc::{EXIT_FAILURE, EXIT_SUCCESS},
    sys::{
        signal::{kill, Signal::SIGINT},
        time::TimeSpec,
        wait::wait,
    },
    time::{clock_gettime, ClockId},
    unistd::{ForkResult, Pid},
    Error,
};
use std::env;

const NLOOP_FOR_ESTIMATION: usize = 1_000_000_000;
const NSECS_PER_MSEC: usize = 1_000_000;
const NSECS_PER_SEC: usize = 1_000_000_000;

#[inline]
fn load(nloop: usize) {
    for _ in 0..nloop {}
}

fn child_fn(
    id: usize,
    buf: &mut Vec<TimeSpec>,
    nrecord: usize,
    nloop_per_resol: usize,
    start: TimeSpec,
) {
    for _ in 0..nrecord {
        load(nloop_per_resol);
        buf.push(get_time());
    }
    for i in 0..nrecord {
        println!(
            "{}\t{}\t{}",
            id,
            diff_nsec(start, buf[i]) / NSECS_PER_MSEC,
            (i + 1) * 100 / &nrecord
        );
    }
    std::process::exit(EXIT_SUCCESS);
}

#[inline]
fn get_time() -> TimeSpec {
    match clock_gettime(ClockId::CLOCK_MONOTONIC) {
        Ok(time) => time,
        Err(e) => {
            eprintln!("clock_gettime() failed {:?}", e);
            std::process::exit(EXIT_FAILURE);
        }
    }
}

#[inline]
fn diff_nsec(before: TimeSpec, after: TimeSpec) -> usize {
    (after.tv_sec() as usize * NSECS_PER_SEC + after.tv_nsec() as usize)
        - (before.tv_sec() as usize * NSECS_PER_SEC + before.tv_nsec() as usize)
}

/// CPU時間を1ms使う処理に必要な計算の量を推定する
fn loops_per_msec() -> Result<usize, Error> {
    let before = get_time();

    // リリースビルドしてしまうと最適化がかかり機能しなくなる
    for _ in 0..NLOOP_FOR_ESTIMATION {}

    let after = get_time();

    // ループ回数をかかった時間(diff_nsec) でわり、1nsあたりのループ回数を計算し、NSECS_PER_MSECを掛け、単位をmsにする
    Ok(NLOOP_FOR_ESTIMATION * NSECS_PER_MSEC / diff_nsec(before, after))
}

#[inline]
fn arg_validation(arg: &String, argname: &str) -> usize {
    match arg.parse::<usize>() {
        Ok(a) => {
            if a < 1 {
                eprintln!("<{}>({}) should be >= 1", argname, arg);
                std::process::exit(EXIT_FAILURE);
            }
            a
        }
        Err(e) => {
            eprintln!("<{}>({}) should be number: {:?}", argname, arg, e);
            std::process::exit(EXIT_FAILURE);
        }
    }
}

/// # コマンドライン引数
/// 第1引数（nproc）: 同時に動かすプロセス数
/// 第2引数（total）: プログラムを動作させる合計時間（ms単位）
/// 第3引数（resol）: 統計情報の採取間隔（ms単位）
fn main() {
    let ret;
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

    let nloop_per_resol = match loops_per_msec() {
        Ok(msec) => msec * resol,
        Err(e) => {
            eprintln!("gettime {:?}", e);
            std::process::exit(EXIT_FAILURE);
        }
    };

    let mut logbuf: Vec<TimeSpec> = Vec::<TimeSpec>::with_capacity(nrecord);
    let mut pids = Vec::<Pid>::with_capacity(nproc);

    let start = get_time();

    let mut ncreated = 0;
    for i in 0..nproc {
        ncreated += 1;
        match unsafe { nix::unistd::fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                pids.push(child);
            }
            Ok(ForkResult::Child) => {
                child_fn(i, &mut logbuf, nrecord, nloop_per_resol, start);
            }
            Err(_) => eprintln!("fork() failed."),
        }
    }

    ret = EXIT_SUCCESS;

    if ret == EXIT_FAILURE {
        for i in 0..ncreated {
            if let Err(e) = kill(pids[i], SIGINT) {
                eprintln!("kill({}) failed: {:?}", &pids[i], e);
            }
        }
    }

    for _ in 0..ncreated {
        if let Err(e) = wait() {
            eprintln!("wait() failed: {:?}", e);
        }
    }

    std::process::exit(ret);
}
