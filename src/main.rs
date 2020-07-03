// https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#summary
//
// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in a company. For example, “Add Sally to
// Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all
// people in a department or all people in the company by department, sorted
// alphabetically.

use std::collections::HashMap;

// Ignoring an actual "interface" since that sort of I/O is later in the book.
fn main() {
    let mut cogs = HashMap::new();
    let cmds = [
        "Add Sally to Engineering",
        "Add Amir to Sales",
        "List Sales",
        "List All",
    ];

    // This is nightmare code. Abusing .expect() to get Some from the Option
    // returned by .next(). Abusing the use of an iterator via artisinal, hand
    // cranked .next() calls. This is terrible. For now, I'm focusing on just
    // getting basic code to run … at all.
    for c in cmds.iter() {
        let mut c_iter = c.split_whitespace();
        let cmd = c_iter.next().expect("");

        if cmd == "Add" {
            let name = c_iter.next().expect("");
            c_iter.next();
            let dept = c_iter.next().expect("");
            println!("+ Adding {} to {}", name, dept);
            cogs.insert(name, dept);
        }

        if cmd == "List" {
            let dept = c_iter.next().expect("");
            println!("> Finding everyone in {}", dept);
            for (k, v) in cogs.iter() {
                if v == &dept || dept == "All" {
                    println!("{} - {}", k, v);
                }
            }
        }
    }
}
