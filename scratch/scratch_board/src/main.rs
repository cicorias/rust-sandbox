fn main() {
    let my_number = 100;
    println!("{}", my_number as u8 as char);

    scratch_board::src::main::run();
    scratch_board::src::main::print_ascii_table();
}

mod scratch_board {
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
