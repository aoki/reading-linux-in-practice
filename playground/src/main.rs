fn main() {
    loop {
        nix::unistd::getppid();
    }
}
