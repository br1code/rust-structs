// To understand when we might want to use structs, let's write a program that calculates the area of a rectangle.
// We'll start with single variables, and then refactor the program until we're using structs instead.

// Let's make a program that will take the width and height of a rectangle specified in pixels
// and calculate the area of the rectangle.

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// Even though it works and figures out the area of the rectangle by calling the area function with each dimension, we can do better.

// The issue with this code is evident in the signature of area:
// fn area(width: u32, height: u32) -> u32 {

// The area function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters.
// The parameters are related, but that’s not expressed anywhere in our program.
// It would be more readable and more manageable to group width and height together.

// One way to do it could be by using tuples:

// Refactoring with Tuples ---
fn main_with_tuples() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_tuple(rect1)
    );
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// In one way, this program is better. Tuples let us add a bit of structure, and we’re now passing just one argument.
// But in another way, this version is less clear: tuples don’t name their elements, so our calculation has become more
// confusing because we have to index into the parts of the tuple.

// It doesn’t matter if we mix up width and height for the area calculation, but if we want to draw the rectangle on the screen,
// it would matter! We would have to keep in mind that width is the tuple index 0 and height is the tuple index 1.

// Refactoring with Structs: Adding More Meaning ---

// We use structs to add meaning by labeling the data.
// We can transform the tuple we’re using into a data type with a name for the whole as well as names for the parts.

struct Rectangle {
    width: u32,
    height: u32,
}

fn main_using_structs() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_using_struct(&rect1)
    );
}

// it receives an immutable borrow of a struct Rectangle instance
fn area_using_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
// We want to borrow the struct rather than take ownership of it.
// This way, main_using_structs retains its ownership and can continue using rect1, which is the reason
// we use the & in the function signature and where we call the function.

// Adding Useful Functionality with Derived Traits ---
// It’d be nice to be able to print an instance of Rectangle while we’re debugging our program and see the values for all its fields.
fn main_2() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1); // ERROR <--- error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
}

// If we try to print an instance of a Rectangle (or any struct), we'll get the following error:
// error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`

// The println! macro can do many kinds of formatting, and by default, the curly brackets
// tell println! to use formatting known as Display: output intended for direct end user consumption.
// The primitive types we’ve seen so far implement Display by default, because there’s only one way you’d
// want to show a 1 or any other primitive type to a user.

// If we continue reading the errors, we’ll find this helpful note:
// = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
// = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

// Let’s try it! The println! macro call will now look like println!("rect1 is {:?}", rect1);.
// Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug.
// The Debug trait enables us to print our struct in a way that is useful for developers so we can
// see its value while we’re debugging our code.
fn main_3() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1); // ERROR <--- error[E0277]: `Rectangle` doesn't implement `std::fmt::Debug`
}
// Compile the code with this change. Drat! We still get an error:
// error[E0277]: `Rectangle` doesn't implement `std::fmt::Debug`

// But again, the compiler gives us a helpful note:
// = help: the trait `std::fmt::Debug` is not implemented for `Rectangle`
// = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`

// Rust does include functionality to print out debugging information, but we have to explicitly opt in
// to make that functionality available for our struct.
// To do that, we add the annotation #[derive(Debug)] just before the struct definition
#[derive(Debug)] // <---- here
struct Rectangle2 {
    width: u32,
    height: u32,
}

fn main_4() {
    let rect1 = Rectangle2 {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
// Now when we run the program, we won’t get any errors, and we’ll see the following output:
// rect1 is Rectangle { width: 30, height: 50 }

// Nice! It’s not the prettiest output, but it shows the values of all the fields for this instance,
// which would definitely help during debugging.

// When we have larger structs, it’s useful to have output that’s a bit easier to read;
// in those cases, we can use {:#?} instead of {:?} in the println! string.

// When we use the {:#?} style in the example, the output will look like this:
// rect1 is Rectangle {
//     width: 30,
//     height: 50,
// }

// Rust has provided a number of traits for us to use with the derive annotation that can add useful behavior to our custom types.
