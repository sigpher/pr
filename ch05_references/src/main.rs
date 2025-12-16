use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

static mut STASH: &i32 = &128;

fn main() {
    // let mut table = Table::new();
    // table.insert(
    //     "Gesualdo".to_string(),
    //     vec![
    //         "many madrigals".to_string(),
    //         "Tenebrae Responsoria".to_string(),
    //     ],
    // );
    // table.insert(
    //     "Caravaggio".to_string(),
    //     vec![
    //         "The Musicians".to_string(),
    //         "The Calling of St. Matthew".to_string(),
    //     ],
    // );
    // table.insert(
    //     "Cellini".to_string(),
    //     vec![
    //         "Perseus with the head of Medusa".to_string(),
    //         "a salt cellar".to_string(),
    //     ],
    // );
    // table.insert(
    //     "Gesualdo".to_string(),
    //     vec![
    //         "many madrigals".to_string(),
    //         "Tenebrae Responsoria".to_string(),
    //     ],
    // );
    // table.insert(
    //     "Caravaggio".to_string(),
    //     vec![
    //         "The Musicians".to_string(),
    //         "The Calling of St. Matthew".to_string(),
    //     ],
    // );
    // table.insert(
    //     "Cellini".to_string(),
    //     vec![
    //         "Perseus with the head of Medusa".to_string(),
    //         "a salt cellar".to_string(),
    //     ],
    // );
    // show(&table);
    // println!("-------------------------------------------");
    // sort_works(&mut table);
    // show(&table);

    let x = 10;
    let y = 20;

    let mut r = &x;
    println!("{}", *r);
    r = &y;
    println!("{}", *r);

    struct Point {
        x: i32,
        y: i32,
    };

    let point = Point { x: 1000, y: 729 };

    let r = &point;
    let rr = &r;
    let rrr = &rr;

    assert_eq!(rrr.y, 729);

    static A: i32 = 10;
    f(&A);
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("{artist}");
        for work in works {
            println!("\t {work}");
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_, works) in table {
        works.sort();
    }
}

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}
