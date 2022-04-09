fn main() {
    let p = std::ptr::null_mut::<i32>();
    println!("before invalid access");
    unsafe {
        p.write(0);
    }
    println!("after invalid access");
    std::process::exit(nix::libc::EXIT_SUCCESS);
}
