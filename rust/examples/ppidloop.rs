fn main() {
    println!("Get PPID. Ctrl + C to stop. ");
    loop {
        nix::unistd::getppid();
    }
}
