// tests/unfurl.rs - Fiddleheads unfurl in sunlight

extern crate fern_lib;
use fern_lib::{Fern, run_simulation};

#[test]
fn test_fiddlehead_unfurling() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.002
    };
    run_simulation(&mut fern, 100);
    println!("The fern now has size: {}", fern.size);
}
