
fn main() {
    let my_number = 10;
    let some_variable = match my_number {
        10 => Some(8),
        _ => Some(0),
    };

    println!(
        "My number is: {my_number}, and the some variable is: {:?}",
        some_variable
    );
}


// fn main() {
//     let my_number = 5;
//     let second_number = match my_number {
//         0 => 0,
//         5 => 10,
//         _ => 2,
//     };

//     println!("My number is: {my_number}, and the second number is: {second_number}");
// }

// fn main() {
//     let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
//     println!(
//         "Inside the tuple is: First item: {:?}
// Second item: {:?}
// Third item: {:?}
// Fourth item: {:?}
// Fifth item: {:?}
// Sixth item: {:?}",
//         random_tuple.0,
//         random_tuple.1,
//         random_tuple.2,
//         random_tuple.3,
//         random_tuple.4,
//         random_tuple.5
//     );

// }




// fn main() {
//     let mut num_vec = Vec::new();  // with_capacity
//     // num_vec.reserve(8);  // reserve space for 5 elements
//     println!("{}", num_vec.capacity());

//     num_vec.push('a');
//     println!("{}", num_vec.capacity());
//     num_vec.push('a');
//     num_vec.push('a');
//     num_vec.push('a');
//     println!("{}", num_vec.capacity());
//     num_vec.push('a');
//     println!("{}", num_vec.capacity());
// }

// macro_rules! print_type {
//     ($val:expr) => {{
//         fn type_of<T>(_: &T) -> &'static str {
//             std::any::type_name::<T>()
//         }
//         println!("Type of `{}` is: {}", stringify!($val), type_of(&$val));
//     }};
// }

// fn main() {
//     let x = 123;
//     let y = "hello";
//     let z = vec![1, 2, 3];

//     print_type!(x);
//     print_type!(y);
//     print_type!(z);
// }
// fn main() {
//     let name1 = String::from("Windy");
//     let name2 = String::from("Gomesy");
//     let mut my_vec = Vec::new();
//     my_vec.push(name1);
//     my_vec.push(name2);

//     println!("My vector: {:?}", my_vec);
//     print_type!(my_vec);
// }

// fn main() {
//     let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

//     let two_to_five = &array_of_ten[2..5];
//     let start_at_one = &array_of_ten[1..];
//     let end_at_five = &array_of_ten[..5];
//     let everything = &array_of_ten[..];

//     println!("Two to five: {two_to_five:?},
// Start at one: {start_at_one:?},
// End at five: {end_at_five:?},
// Everything: {everything:?}");
// }
