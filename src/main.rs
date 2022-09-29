use is_ci::uncached;

pub fn main() {
    let is_ci = uncached();
    if is_ci {
        std::process::exit(0);
    }
    std::process::exit(1);
}
