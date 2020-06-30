#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use std::fmt::Display;
use std::collections::HashSet;

fn main() {
    let mut a = Appellation { name: String::from("Zeus"),
                              nicknames: vec![String::from("cloud collector"),
                                              String::from("king of the gods")] };
    println!("before assignment");
    a = Appellation { name: String::from("Hera"), nicknames: vec![] };
    println!("at end of block");

    let boxed_lunch: RcBox<String> = RcBox {
        ref_count: 1,
        value: String::from("lunch"),
    };

    let boxed_displayable: &RcBox<dyn Display> = &boxed_lunch;
    display(boxed_displayable);

    let mut s = Selector { elements: vec!['x', 'y', 'z'],
                           current: 2 };

    // Because `Selector` implements `Deref`, we can use the `*` operator to
    // refer to its current element.
    assert_eq!(*s, 'z');

    // Assert that 'z' is alphabetic, using a method of `char` directly on a
    // `Selector`, via deref coercion.
    assert!(s.is_alphabetic());

    // Change to 'z' to a 'w', by assigning to the `Selector`'s referent.
    *s = 'w';

    assert_eq!(s.elements, ['x', 'y', 'w']);

    let s = Selector { elements: vec!["good", "bad", "ugly"],
                       current: 2, };
    show_it(&s);
    show_it_generic(&s as &str);

    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>)
        = squares.iter().partition(|&n| n & (n-1) == 0);

    assert_eq!(powers_of_two.len(), 3);
    assert_eq!(impure.len(), 4);

    let (upper, lower): (String, String)
        = "Great Teacher Onizuka".chars().partition(|&c| c.is_uppercase());
    assert_eq!(upper, "GTO");
    assert_eq!(lower, "reat eacher nizuka");

    println!("{:?}", ping(Ipv4Addr::new(23, 21, 68, 141))); // pass an Ipv4Addr
    println!("{:?}", ping([66, 146, 219, 98]));             // pass a [u8; 4]
    println!("{:?}", ping(0xd076eb94_u32));                 // pass a u32

    let text = String::from("Beautiful Soup");
    let bytes: Vec<u8> = text.into();
}

struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T,
}

fn display(boxed: &RcBox<dyn Display>) {
    println!("For your enjoyment: {}", &boxed.value);
}

struct Selector<T> {
    /// Elements available in this `Selector`.
    elements: Vec<T>,

    /// The index of the "current" element in `elements`. A `Selector`
    /// behaves like a pointer to the current element.
    current: usize,
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

fn show_it(thing: &str) { println!("{}", thing); }
fn show_it_generic<T: Display>(thing: T) { println!("{}", thing); }

use std::net::Ipv4Addr;
fn ping<A>(address: A) -> std::io::Result<bool>
    where A: Into<Ipv4Addr>
{
    let _ipv4_address = address.into();
    Ok(true)
}
