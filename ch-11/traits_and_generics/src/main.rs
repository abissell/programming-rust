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
