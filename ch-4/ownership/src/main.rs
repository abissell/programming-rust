fn main() {
    print_padovan();

    {
        let point = Box::new((0.625, 0.5)); // point allocated here
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(label, "(0.625, 0.5)");
        println!("{:?}", label);
    }

    composers();

    vector_moves();

    rc();
}

fn composers() {
    struct Person { name: String, birth: i32 }

    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(),
                            birth: 1525 });
    composers.push(Person { name: "Dowland".to_string(),
                            birth: 1563 });
    composers.push(Person { name: "Lully".to_string(),
                            birth: 1632 });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}

fn print_padovan() {
    let mut padovan = vec![1,1,1]; // allocated here
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}                                 // dropped here

fn vector_moves() {
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    // 1. Pop a value off the end of the vector:
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    // 2. Move a value out of the middle of the vector, and move the last
    // element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // Let's see what's left of our vector.
    assert_eq!(v, vec!["101", "104", "substitute"]);
}

use std::rc::Rc;
fn rc() {
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t = s.clone();
    let u = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);
}
