#![allow(dead_code, unused_variables, unused_mut)]

fn main() {
    println!("Chapter 5!");
}

struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

/// [p. 85]:(https://doc.rust-lang.org/book/ch05-01-defining-structs.html#defining-and-instantiating-structs)
/// Listing 5-4: This is a strange way to show a constructor pattern...
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

/// Normally you would see this instead... this is best practice:
impl User {
    pub fn new(email: String, username: String) -> Self {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}

/// Shorthand syntax (still on p. 85): Notice that the two following functions are equivalent.
fn no_shorthand(email: String, username: String) -> User {
    // Clippy or your IDE will complain because you don't need to repeat email and username.
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

/// It is more idiomatic to use this shorthand:
fn shorthand(email: String, username: String) -> User {
    // Now Clippy and your IDE should be happy.
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

/// Shorthand struct copying syntax (Struct update syntax)
/// [p. 86](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax)
fn increment_sign_in_count(user: User) -> User {
    // We can copy the fields from one User struct to another with shorthand:
    User {
        sign_in_count: user.sign_in_count + 1,
        ..user
    }
}

struct SuperUser {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
    permission_level: u16,
}

/// It would be nice if I could do this, but I can't :(
// fn super_user_from_user(user: User) -> SuperUser {
//     SuperUser{
//         permission_level: 2,
//         ..user
//     }
// }

// Tuple Structs
// [p. 86](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax)

/// A point consisting of X, Y and Z
struct Point(u64, u64, u64);

/// A color consisting of Red, Green and Blue
struct Color(u64, u64, u64);

fn use_point(point: &Point) {
    println!("X is {}", point.0);
    println!("Y is {}", point.1);
    println!("Z is {}", point.2);
}

fn use_color(color: &Color) {
    println!("Red is {}", color.0);
    println!("Green is {}", color.1);
    println!("Blue is {}", color.2);
}

/// `Point` and `Color` can **NOT** be used interchangeably
fn not_interchangeable() {
    let point = Point(1, 2, 3);
    let color = Color(1, 2, 3);

    // No! doesn't work
    // use_point(&color);
    // use_color(&point);

    // Yes! this works
    use_point(&point);
    use_color(&color);
}
