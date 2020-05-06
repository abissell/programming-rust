use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;
fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(),
                      "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(),
                      "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                 vec!["Perseus with the head of Medusa".to_string(),
                      "a salt cellar".to_string()]);
    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");

    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(rx == ry); // the referents are equal
    assert!(!std::ptr::eq(rx, ry)); // but occupy different addresses

    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);

    reference_safety();

    use_smallest();

    structs_containing_references();

    use_stringtable();

    sharing_versus_mutation();
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn factorial(n: usize) -> usize {
    (1..n+1).fold(1, |a, b| a * b)
}

static mut STASH: &i32 = &128;
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn reference_safety() {
    {
        let r;
        {
            let x = 1;
            r = &x;
            assert_eq!(*r, 1);
        }
    }

    f(&5);
}

fn use_smallest() {
    let parabola = [9, 4, 1, 0, 1, 4, 9];
    let s = smallest(&parabola);
    assert_eq!(*s, 0);
}

fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r; }
    }
    s
}

struct S<'a> {
    r: &'a i32
}

fn structs_containing_references() {
    {
        let x = 10;
        let s = S { r: &x };
        assert_eq!(*s.r, 10);
    }
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0 .. self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

fn use_stringtable() {
    let st = StringTable { elements: vec!["hello".to_string(), "world".to_string()] };
    let found = st.find_by_prefix("hell");
    assert_eq!(found, Some(&"hello".to_string()));
    let not_found = st.find_by_prefix("heaven");
    assert_eq!(not_found, None);
}

fn sharing_versus_mutation() {
    let v = vec![4, 8, 19, 27, 34, 10];
    {
        let r = &v;
        r[0];
    }
    let _aside = v;
}
