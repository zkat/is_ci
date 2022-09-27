use is_ci::uncached;

pub fn main() {
    let is_ci = uncached();
    println!("{}", is_ci);
}
