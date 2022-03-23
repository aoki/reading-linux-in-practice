// does not work
fn main() {
    let p: *mut i32 = std::ptr::null();
    println!("before invalid access");
    let q: *mut i32 = 1 as *mut i32;
    unsafe {
        p = q;
    }
    println!("after invalid access");
    std::process::exit(nix::libc::EXIT_SUCCESS);
}
