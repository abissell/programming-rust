use fern_lib::Fern;
fn main() {
    println!("Hello, world!");
    let fern = Fern {
        size: 3.0,
        growth_rate: 0.001
    };
    println!("The library fern size: {}", fern.size);
}
