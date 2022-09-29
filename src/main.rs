use is_ci::uncached;

pub fn main() {
    if let is_ci = uncached() {
        std::process::exit(0);
    }
    std::process::exit(1);
}
