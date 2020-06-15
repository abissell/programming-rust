#![allow(dead_code)]

fn main() {
    use std::fs::File;
    let mut local_file = File::create("hello.txt").expect("Couldn't create file");
    say_hello(&mut local_file).expect("Couldn't write to local file");

    let mut bytes = vec![];
    say_hello(&mut bytes).expect("Couldn't write to byte buffer");
    assert_eq!(bytes, b"hello world\n");

    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"hello").expect("Couldn't write to second byte buffer");

    let _writer: &mut dyn Write = &mut buf;

    // let _w: Box<dyn Write> = Box::new(local_file);

    say_hello_generic(&mut local_file).expect("Couldn't write to byte buffer");
    say_hello_generic(&mut bytes).expect("Couldn't write to byte buffer");
    assert_eq!(bytes, b"hello world\nhello world\n");

    let _v1 = (0 .. 1000).collect::<Vec<i32>>();

    let mut sink = std::io::sink();
    say_hello(&mut sink).expect("Couldn't say hello to sink");

    assert_eq!('$'.is_emoji(), false);

    let zero = 0; // type unspecified; could be `i8`, `u8`, ...
    // zero.abs(); // error: method `abs` not found
    i64::abs(zero); // ok
}

use std::io::{Write, Result};

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

/// Given two values, pick whichever one is less.
fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}

fn say_hello_generic<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

trait Vegetable {}

struct Salad {
    veggies: Vec<Box<dyn Vegetable>>
}

struct Canvas {}

impl Canvas {
    fn write_at(&self, _x: i32, _y: i32, _c: char) {

    }
}

/// A trait for characters, items, and scenery -
/// anything in the game world that's visible on screen.
trait Visible {
    /// Render this object on the given canvas.
    fn draw(&self, canvas: &mut Canvas);

    /// Return true if clicking at (x, y) should
    /// select this object.
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

struct Broom {
    x: i32,
    y: i32,
    height: i32,
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.y - self.height - 1 .. self.y {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x
            && self.y - self.height - 1 <= y
            && y <= self.y
    }
}

/// A Writer that ignores whatever data you write to it.
pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // Claim to have successfully written the whole buffer.
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

/// Implement IsEmoji for the built-in character type.
impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        false
    }
}

use serde::Serialize;
use serde_json;
use std::collections::HashMap;

pub fn save_configuration(config: &HashMap<String, String>)
    -> std::io::Result<()>
{
    // Create a JSON serializer to write the data to a file.
    use std::fs::File;
    let writer = File::create("some_config_filename")?;
    let mut serializer = serde_json::Serializer::new(writer);

    // The serde `.serialize()` method does the rest.
    config.serialize(&mut serializer)?;
    Ok(())
}

pub trait Spliceable {
    fn splice(&self, other: &Self) -> Self;
}

trait StringSet {
    /// Return a new empty set.
    fn new() -> Self;

    /// Return a set that contains all the strings in `strings`.
    fn from_slice(strings: &[&str]) -> Self;

    /// Find out if this set contains a particular `value`.
    fn contains(&self, string: &str) -> bool;

    /// Add a string to this set.
    fn add(&mut self, string: &str);
}

/// Return the set of words in `document` that aren't in `wordlist`.
fn unknown_words<S: StringSet>(document: &Vec<String>, wordlist: &S) -> S {
    let mut unknowns = S::new();
    for word in document {
        if !wordlist.contains(word) {
            unknowns.add(word);
        }
    }
    unknowns
}

/// Loop over an iterator, storing the values in a new vector.
fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
    let mut results = Vec::new();
    for value in iter {
        results.push(value);
    }
    results
}

/// Print out all the values produced by an iterator
fn dump<I>(iter: I)
    where I: Iterator, I::Item: std::fmt::Debug
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

/// Print out all the values produced by a String iterator
fn dump_strings(iter: &mut dyn Iterator<Item=String>)
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);
    }
}

trait Pattern {
    type Match;

    fn search(&self, string: &str) -> Option<Self::Match>;
}

/// You can search a string for a particular character.
impl Pattern for char {
    /// A "match" is just the location where the
    /// character was found.
    type Match = usize;

    fn search(&self, _string: &str) -> Option<usize> {
        // Find the value in the string & return position
        Some(2)
    }
}

use std::ops::{Add, Mul};

fn dot<N>(v1: &[N], v2: &[N]) -> N
    where N: Add<Output=N> + Mul<Output=N> + Default + Copy
{
    let mut total = N::default();
    for i in 0 .. v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_dot() {
    assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}
