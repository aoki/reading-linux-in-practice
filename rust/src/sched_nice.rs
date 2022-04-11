use nix::libc::EXIT_FAILURE;
use std::{env, process::exit};

#[inline]
/// 与えられた引数を `usize` に変換します。
/// 変換に失敗した場合は、エラーを表示し終了します。
fn arg_validation(arg: &String, argname: &str) -> usize {
    match arg.parse::<usize>() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("<{}>({}) should be >= 1: {:?}", argname, arg, e);
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
    let nrecord = total / resol;

    todo!();
}
