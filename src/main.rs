use jiff::Zoned;

fn main() {
    let now = Zoned::now();

    println!("{}", now);
}
