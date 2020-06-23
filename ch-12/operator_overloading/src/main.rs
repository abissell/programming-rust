#![allow(dead_code)]

fn main() {
    let x = Complex { re: 5, im: 2 };
    let y = Complex { re: 2, im: 5 };
    assert_eq!(x * y, Complex { re: 0, im: 29 });

    let s = "d\x6fv\x65t\x61i\x6c".to_string();
    let t = "\x64o\x76e\x74a\x69l".to_string();
    assert!(s == t);

    assert_eq!(format!("{} {}", s, t), "dovetail dovetail");

    assert!(Interval { lower: 10, upper: 20 } < Interval { lower: 20, upper: 40 });
    assert!(Interval { lower: 7, upper: 8 } >= Interval { lower: 0, upper: 1 });
    assert!(Interval { lower: 7, upper: 8 } <= Interval { lower: 7, upper: 8 });

    // Overlapping intervals aren't ordered with respect to each other.
    let left = Interval { lower: 10, upper: 30 };
    let right = Interval { lower: 20, upper: 40 };
    assert!(!(left < right));
    assert!(!(left >= right));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Complex<T> {
    /// Real portion of the complex number
    re: T,

    /// Imaginary portion of the complex number
    im: T,
}

use std::ops::Add;

impl<L, R, O> Add<Complex<R>> for Complex<L>
    where L: Add<R, Output=O>
{
    type Output = Complex<O>;
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

use std::ops::Neg;

impl<T, O> Neg for Complex<T>
    where T: Neg<Output=O>
{
    type Output = Complex<O>;
    fn neg(self) -> Complex<O> {
        Complex { re: -self.re, im: -self.im }
    }
}

use std::ops::AddAssign;

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

use std::ops::Mul;

impl<L, R, O> Mul<Complex<R>> for Complex<L>
    where L: Mul<R, Output=O>, L: Copy, R: Copy, O: Add<O, Output=O>, O: Neg<Output=O>
{
    type Output = Complex<O>;
    fn mul(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            re: self.re * rhs.re + -(self.im * rhs.im),
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T, // inclusive
    upper: T, // exclusive
}

use std::cmp::{Ordering, PartialOrd};

impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other { Some(Ordering::Equal) }
        else if self.lower >= other.upper { Some(Ordering::Greater) }
        else if self.upper <= other.lower { Some(Ordering::Less) }
        else { None }
    }
}

struct Image<P> {
    width: usize,
    pixels: Vec<P>
}

impl<P: Default + Copy> Image<P> {
    /// Create a new image of the given size.
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * height]
        }
    }
}

impl<P> std::ops::Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row: usize) -> &[P] {
        let start = row * self.width;
        &self.pixels[start .. start + self.width]
    }
}

impl<P> std::ops::IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut [P] {
        let start = row * self.width;
        &mut self.pixels[start .. start + self.width]
    }
}
