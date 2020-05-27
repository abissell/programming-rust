#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    use std::mem::size_of;
    assert_eq!(size_of::<Ordering>(), 1);
    assert_eq!(size_of::<HttpStatus>(), 2);

    assert_eq!(HttpStatus::Ok as i32, 200);

    use self::BinaryTree::*;
    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));
    let mercury_tree = NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: Empty,
        right: Empty,
    }));
    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree,
    }));
    let mut tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: Empty,
    }));
    tree.add("Pluto");
    tree.add("Venus");

    assert_eq!(rough_time_to_english(RoughTime::InThePast(TimeUnit::Hours, 1)), "an hour ago");
    assert_eq!(rough_time_to_english(RoughTime::InThePast(TimeUnit::Days, 1)), "a day ago");
    assert_eq!(rough_time_to_english(RoughTime::InThePast(TimeUnit::Days, 2)), "2 days ago");
    assert_eq!(rough_time_to_english(RoughTime::JustNow), "just now");
    assert_eq!(rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Hours, 1)), "an hour from now");
    assert_eq!(rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Days, 1)), "a day from now");
    assert_eq!(rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Days, 2)), "2 days from now");

    assert_eq!(describe_point(0, 0), "at the origin");
    assert_eq!(describe_point(0, 3), "on the y axis");

    let mut hello_chars = "hello".chars().peekable();
    let at_end =
        match hello_chars.peek() {
            Some(&'\r') | Some(&'\n') | None => true,
            _ => false,
        };
}

enum Ordering {
    Less,
    Equal,
    Greater,
}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years
}

impl TimeUnit {
    /// Return the plural noun for this time unit.
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    /// Return the singular noun for this time unit.
    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

/// A timestamp that has been deliberately rounded off, so our program
/// says "6 months ago" instead of "Feburary 9, 2016, at 9:49 AM"
#[derive(Copy, Clone, Debug, PartialEq)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(TimeUnit::Hours, 1) =>
            format!("an hour ago"),
        RoughTime::InThePast(unit, 1) =>
            format!("a {} ago", unit.singular()),
        RoughTime::InThePast(units, count) =>
            format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow =>
            format!("just now"),
        RoughTime::InTheFuture(TimeUnit::Hours, 1) =>
            format!("an hour from now"),
        RoughTime::InTheFuture(unit, 1) =>
            format!("a {} from now", unit.singular()),
        RoughTime::InTheFuture(units, count) =>
            format!("{} {} from now", count, units.plural()),
    }
}

// An ordered collection of T's
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

// A part of a BinaryTree.
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty =>
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                })),
            BinaryTree::NonEmpty(ref mut node) =>
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
        }
    }
}

fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;
    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x axis",
        (Equal, _) => "on the y axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else",
    }
}
