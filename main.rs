/* This is a "Hello World" style program
 * Highlighting Many Common Programming
 * Functions in the Rust Language. This is
 * to be Used as Reference Material for
 * More Complicated Programs
 *
 * Hard Requirements: Rustc
 * Optional Requirements: make build-essential
 * 
 * To Compile run the following command on *nix
 * rustc main.rs -o HelloWorld.bin
 * 
 * Alternatively create and use a Makefile
 * Run the following commands in the working directory
 * on *nix to create the Makefile
 * touch Makefile
 * echo build: > Makefile
 * echo -e '\t'rustc main.rs -o HelloWorld.bin
 *
 * To Use the Makefile, Simply Run:
 * make
 * This will require make to be installed.
 *
 * Creator: Christopher Prats
 * License: GPLv2
 */
//The Main Function, AKA the Entry Point
fn main() {
    // Set the Value of run_decision to
    // the return value of the to_run function
    let run_decision = to_run();
    // Call the Hello World function with the
    // run_decision as a parameter
    hello_world(run_decision);
}
// The to_run function. This function has a return
// value, similar to int functions in other languages
fn to_run() -> i32 {
    // Create a Variable to Store Input. Rust input is
    // more akin to a Scanner in Java than running scanf
    // in C or std::cin in C++
    let mut input = String::new();
    // Call the show_menu function
    show_menu();
    // Read the User Input and Store it as a String. If There is an Error
    // using stdin, Print "Input Failure" as an Error Message
    std::io::stdin().read_line(&mut input).expect("Input Failure");
    // Convert the Input in to a 32bit Integer. If the Input Can Not be
    // Converted, Print "Please Enter an Integer"
    let _parsed_input: i32 = input.trim().parse().expect("Please Enter an Integer");
    // Return the Value of the _parsed_input Variable
    _parsed_input
}
// The hello_world function. This function lacks a return
// value, similar to void functions in other languages.
// Additionally, the function accepts a 32 bit integers as
// a parameter
fn hello_world(run: i32) {
    // If run is equal to 1 print
    // "Hello World!"
    if run == 1 {
        println!("Hello World!");
    }
    // If run is equal to 2 print,
    // "Goodbye World!"
    else if run == 2 {
        println!("Goodbye World!");
    }
    // If run is not equal to 1 or 2,
    // print "Aloha World!"
    else {
        println!("Aloha World!");
    }
}
// The show_menu function. This Function Demonstrates Basic Text Formatting for
// stdout in Rust. "\n" Creates a New Line, and \t Creates a Tab
fn show_menu() {
    println!("\nAvailable Options:");
    println!("\t1. Display Hello Statement");
    println!("\t2. Display Goodby Statement");
    println!("\tOther Integers: Display Aloha Statement");
    println!("Select an Operation: ");
}
