use nix::libc::EXIT_SUCCESS;

fn main() {
    let ret: i32 = EXIT_SUCCESS;
    std::process::exit(ret)
}
