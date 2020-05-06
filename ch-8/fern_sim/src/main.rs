mod spores;
use spores::Spore;

pub mod plant_structures;
use plant_structures::leaves;
use plant_structures::roots;
use plant_structures::stems;

fn main() {
    println!("Hello, world!");
    let _spore = Spore { msg: "spore!".to_string() };
    spores::produce_spore();

    let _leaf = leaves::Leaf { msg: "leaf!".to_string() };
    let _root = roots::Root { msg: "root!".to_string() };
    let _stem = stems::Stem { msg: "stem!".to_string() };
}
