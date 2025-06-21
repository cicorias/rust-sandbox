
fn main() {
    let country = String::from("Austria");
    adds_hungary(country);
    // println!("{} is a country in Europe.", country);
}
 
fn adds_hungary(mut string_to_add_hungary_to: String) {
    string_to_add_hungary_to.push_str("-Hungary");
    println!("{}", string_to_add_hungary_to);
}


/* 
// dereferencing and references
fn main() {
    let mut my_number = 8;
    let num_ref = &mut my_number;
    *num_ref += 10;
    println!("{}", my_number);
 
    let second_number = 800;
    let triple_reference = &&&second_number;
    println!("Are they equal? {}", second_number == ***triple_reference);
}
*/

/*
// example display and feature flags
// If the crate is compiled with `--features allow_dead` we silence the lint,
// otherwise we keep the usual warning.
#![cfg_attr(feature = "allow_dead", allow(dead_code))]
#![cfg_attr(not(feature = "allow_dead"), warn(dead_code))]

#[derive(Debug)]
struct Person<'q> {
    other_name: String,
    name: &'q str,
    age: u8,
}

fn main() {
    // Create an instance of Person
    let person = Person {
        other_name: String::from("Alice"),
        name: "Bob",
        age: 30,
    };
    let my_number = {
    let second_number = 8;
        second_number + 9
    };
 
    println!("My number is: {:?}", my_number);
    println!("persone: {:#?}", person);
}

*/

/*
fn main() {
    let color1 = "red";
    let color2 = "blue";
    let color3 = "green";
 
    println!("I like {color1} and {} and {color3}", color2);
}
*/



/*
fn main() {
    let str1 = "Hello!";
    println!("str1 is {} bytes and also {} characters.", str1.len(), str1.chars().count());
    let str2 = "안녕!";
    println!("str2 is {} bytes but only {} characters.", str2.len(), str2.chars().count());

    // use my trait to get the length of the string
    println!("str1 slen: {}", str1.char_count());
}

// implement a function that is called slen that provides the count of chars in a string as an extension to the str type

// 1. Define an extension trait
pub trait StrExt {
    /// Returns the number of Unicode scalar values (what `.chars().count()` does)
    fn char_count(&self) -> usize;
}

// 2. Implement it for `str`
impl StrExt for str {
    fn char_count(&self) -> usize {
        self.chars().count()
    }
}

// (Optional) If you want to call `.slen()` on owned `String` as well:
impl StrExt for String {
    fn char_count(&self) -> usize {
        // delegate to the `str` impl
        self.as_str().char_count()
    }
}
*/

// fn main() {
//     println!("Size of a char: {}", std::mem::size_of::<char>());
//     println!("Size of a: {}", "a".len());
//     println!("Size of ß: {}", "ß".len());
//     println!("Size of 国: {}", "国".len());
//     println!("Size of 𓅱: {}", "𓅱".len());

//     println!("{:?}", "a".as_bytes());
//     println!("{:?}", "ß".as_bytes());
//     println!("{:?}", "国".as_bytes());
//     println!("{:?}", "𓅱".as_bytes());

//     println!("{:?}", 'A'.to_digit(16));

// }


/* *
fn main() {
    let my_number = 256;
    println!("{}", my_number as u8);
    println!("{}", my_number as u16);
    // printout a bit representation of the number formatted in chunks of 8 bits
    println!("{:08b}", my_number as u8);
    // printout a bit representation of the number formatted in chunks of 8 bits for 16 bits with a space in between the 8 bits
    println!("{:08b} {:08b}", (my_number >> 8) as u8, (my_number & 0xFF) as u8);


    // local_utils::src::main::run();
    // local_utils::src::main::print_ascii_table();
}
*/


/* * This file is part of the scratch board application.
 * It demonstrates basic Rust functionality and ASCII table printing.
 *
 * The code includes:
 * - A main function that prints a character corresponding to a number.
 * - A module for local utilities that contains functions for running the application
 *   and printing an ASCII table.
 *
mod local_utils {
    pub mod src {
        pub mod main {
            use comfy_table::{Table, presets::UTF8_FULL, Cell};

            /// This is the main module for the scratch board application.
            pub fn run() {
                println!("Running the scratch board application...");
            }

            /// Function to print out an ASCII table in three columns:
            /// Number | Character | Hex
            pub fn print_ascii_table() {
                // Create a new table with a UTF-8 full-border preset
                let mut table = Table::new();
                table.load_preset(UTF8_FULL);

                // Set column headers
                table.set_header(vec![
                    Cell::new("Number"),
                    Cell::new("Character"),
                    Cell::new("Hex"),
                ]);

                // Populate rows for ASCII codes 0–127
                for i in 0u8..128 {
                    let ch = if i.is_ascii_graphic() || i == b' ' {
                        (i as char).to_string()
                    } else {
                        // represent non-printable control codes in caret notation
                        format!("^{}", (i ^ 0x40) as char)
                    };

                    table.add_row(vec![
                        Cell::new(i.to_string()),
                        Cell::new(ch),
                        Cell::new(format!("{:02X}", i)),
                    ]);
                }

                // Print the table to stdout
                println!("{}", table);
            }
        }
    }
}
*/