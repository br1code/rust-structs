fn main() {
    // Structs ------------------------------------------------------------------------

    // A struct, or structure, is a custom data type that lets you name and package together multiple
    // related values that make up a meaningful group.

    // we’ll compare and contrast tuples with structs, demonstrate how to use structs, and discuss how
    // to define methods and associated functions to specify behavior associated with a struct’s data.

    // Structs and enums are the building blocks for creating new types in your program’s domain
    // to take full advantage of Rust’s compile time type checking.

    // Defining and Instantiating Structs ---
    // Structs are similar to tuples
    // Like tuples, the pieces of a struct can be different types.
    // Unlike with tuples, you’ll name each piece of data so it’s clear what the values mean.
    // As a result of these names, structs are more flexible than tuples:
    // you don’t have to rely on the order of the data to specify or access the values of an instance.

    // To define a struct, we enter the keyword struct and name the entire struct
    struct SomeStruct {}

    // Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields.
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields.

    // We create an instance by stating the name of the struct and then add curly brackets containing key: value pairs,
    // where the keys are the names of the fields and the values are the data we want to store in those fields.
    // We don’t have to specify the fields in the same order in which we declared them in the struct.
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // To get a specific value from a struct, we can use dot notation.
    println!("Username: {}", user1.username);

    // If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.
    let mut user2 = User {
        email: String::from("someone@mail.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    user2.email = String::from("someone@example.com");
    // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.

    // As with any expression, we can construct a new instance of the struct as the last expression
    // in the function body to implicitly return that new instance.
    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    // Because the parameter names and the struct field names are exactly the same in Listing 5-4,
    // we can use the field init shorthand syntax to rewrite build_user so that it behaves exactly
    // the same but doesn’t have the repetition of email and username
    fn build_user2(email: String, username: String) -> User {
        User {
            email, // here
            username, // here
            active: true,
            sign_in_count: 1,
        }
    }

    // Creating Instances From Other Instances With Struct Update Syntax ---
    // It’s often useful to create a new instance of a struct that uses most of an old instance’s values but changes some.
    // You’ll do this using struct update syntax.

    // we can do it like this:
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // BUT, this is how we do it using struct update syntax (like spread operator from another languages)
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // Using Tuple Structs without Named Fields to Create Different Types ---
    // Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields;
    // rather, they just have the types of the fields.
    // Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples, and naming each
    // field as in a regular struct would be verbose or redundant.

    // To define a tuple struct, start with the struct keyword and the struct name followed by the types in the tuple.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Note that the black and origin values are different types, because they’re instances of different tuple structs.
    // Each struct you define is its own type, even though the fields within the struct have the same types.

    // Unit-Like Structs Without Any Fields ---
    // You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly
    // to (), the unit type. Unit-like structs can be useful in situations in which you need to implement a trait on some type
    // but don’t have any data that you want to store in the type itself.

    // Ownership of Struct Data ---
    // In the previous User struct definition, we used the owned String type rather than the &str string slice type.
    // This is a deliberate choice because we want instances of this struct to own all of its data and for that data
    // to be valid for as long as the entire struct is valid.

    // Although, it’s possible for structs to store references to data owned by something else, but to do so requires
    // the use of lifetimes, a Rust feature that we’ll discuss later in the near future.
    //  Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

    // Let’s say you try to store a reference in a struct without specifying lifetimes, like this, which won’t work:
    struct UserWrong {
        // username: &str, // ^ expected lifetime parameter
        // email: &str, // ^ expected lifetime parameter
        sign_in_count: u64,
        active: bool,
    }
}
