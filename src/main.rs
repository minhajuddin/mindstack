fn main() {
    if let Err(e) = mindstack::get_args().and_then(mindstack::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
