// Method Syntax ------------------------------------------------------------------------------

// Methods are similar to functions: they’re declared with the fn keyword and their name, they can
// have parameters and a return value, and they contain some code that is run when they’re called from somewhere else.

// Their first parameter is always self, which represents the instance of the struct the method is being called on.

// Defining Methods ---
// Let’s change the area function that has a Rectangle (from the previous example) instance as a parameter and instead
// make an area method defined on the Rectangle struct.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// To define the function within the context of Rectangle, we start an impl (implementation) block.
// Then we move the area function within the impl curly brackets and change the first (and in this case, only) parameter
// to be self in the signature and everywhere within the body.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// In main, where we called the area function and passed rect1 as an argument, we can instead use method syntax to call the
// area method on our Rectangle instance. The method syntax goes after an instance: we add a dot followed by the method name,
// parentheses, and any arguments.
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // HERE
    );
}

// In the signature for area, we use &self instead of rectangle: &Rectangle because Rust knows the type of self is Rectangle
// due to this method’s being inside the impl Rectangle context.

// Note that we still need to use the & before self, just as we did in &Rectangle.
// Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably,
// just as they can any other parameter.

// We’ve chosen &self here for the same reason we used &Rectangle in the function version:
// we don’t want to take ownership, and we just want to read the data in the struct, not write to it.

// If we wanted to change the instance that we’ve called the method on as part of what the method does,
// we’d use &mut self as the first parameter.

// Having a method that takes ownership of the instance by using just self as the first parameter is RARE;
// this technique is usually used when the method transforms self into something else and you want to
// prevent the caller from using the original instance after the transformation.

// Where’s the -> Operator? ---
// In C and C++, two different operators are used for calling methods: you use . if you’re calling a method on the
// object directly and -> if you’re calling the method on a pointer to the object and need to dereference the pointer first.
// In other words, if object is a pointer, object->something() is similar to (*object).something().

// Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing.
// Calling methods is one of the few places in Rust that has this behavior.

// Here’s how it works: when you call a method with object.something(),
// Rust automatically adds in &, &mut, or * so object matches the signature of the method.
// In other words, the following are the same:

// p1.distance(&p2);
// (&p1).distance(&p2);

// The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self.

// Given the receiver and name of a method, Rust can figure out definitively whether the method is reading:
// - (&self)
// - mutating (&mut self)
// - consuming (self)

// The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.

// Methods with More Parameters ---
// Let’s practice using methods by implementing a second method on the Rectangle struct.
// This time, we want an instance of Rectangle to take another instance of Rectangle and return true if
// the second Rectangle can fit completely within self; otherwise it should return false.
fn main_2() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false
}

// We know we want to define a method, so it will be within the impl Rectangle block.
// The method name will be can_hold, and it will take an immutable borrow of another Rectangle as a parameter.

// We can tell what the type of the parameter will be by looking at the code that calls the method:
// rect1.can_hold(&rect2) passes in &rect2, which is an immutable borrow to rect2, an instance of Rectangle.
// This makes sense because we only need to read rect2 (rather than write, which would mean we’d need a mutable borrow),
// and we want main to retain ownership of rect2 so we can use it again after calling the can_hold method.

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Methods can take multiple parameters that we add to the signature after the self parameter, and those parameters
// work just like parameters in functions.

// Associated Functions ---
// Another useful feature of impl blocks is that we’re allowed to define functions within
// impl blocks that don’t take self as a parameter.

// These are called associated functions because they’re associated with the struct.
// They’re still functions, not methods, because they don’t have an instance of the struct to work with.
// Example: String::from

// Associated functions are often used for constructors that will return a new instance of the struct.
// For example, we could provide an associated function that would have one dimension parameter and use
// that as both width and height, thus making it easier to create a square Rectangle
// rather than having to specify the same value twice:

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// To use this function, we use the :: syntax with the struct name.
fn asd() {
    let sq = Rectangle::square(3);
}

// Multiple impl Blocks ---
// Each struct is allowed to have multiple impl blocks.

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// NOTE: there’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax.