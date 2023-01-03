#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

/// # Getting Help from the Compiler (p. 156)
/// You can use this trick when you don't know the type of something. Annotate a `let` statement
/// with some type, even though you know it is wrong, then compile. The compiler will tell you
/// what the actual type is.
mod type_discovery_trick {
    /// Ignore this
    const ARR: [&str; 2] = ["foo", "bar"];

    /// A function that returns a type I'm not sure about
    fn what_type_does_this_return() -> std::slice::Iter<'static, &'static str> {
        ARR.iter()
    }

    /// I can use the compiler to tell me what the exact type is.
    fn function() {
        // What if I need to annotate this type and don't know that type is being returned?
        let iter = what_type_does_this_return();
    }
}

/// # Creating an Error from Scratch
/// We typically use libraries like `Snafu`, but let's make an error from scratch.
mod custom_error {
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    // Technically speaking, anything can be used as an error.
    fn dumb_error_type() -> Result<(), u64> {
        // This is a dumb error type because it is not idiomatic. What do we do with `1`?
        Err(1)
    }

    /// Instead error types should implement the std library `Error` trait. This is still a weird
    /// error, but at least it implements the correct stuff to be a proper Rust error.
    #[derive(Debug)]
    pub struct ErrorMessage(String);

    // Technically nothing is required beyond `Debug`
    impl Display for ErrorMessage {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Display::fmt(&self.0, f)
        }
    }

    impl Error for ErrorMessage {
        // technically there is nothing else we must do here. below we will see how we should
        // actually implement (override) the `source` function.
    }

    /// The above error is fine, but it's missing something important. The `Error` trait allows you
    /// to implement a `source` function which returns the underlying error that is being "wrapped".
    /// This allows for a pseudo-stack-trace of errors. Here's how you can implement a more
    /// functional error.
    #[derive(Debug)]
    pub struct BetterError {
        /// The error message that we want to include. This may be adding context to an underlying
        /// error held in `source`.
        message: String,

        /// The underlying error that is being wrapped by `BetterError` if one exists. Note that it
        /// is very common and idiomatic to require error types to also implement `Send` and `Sync`.
        /// Without `Send` and `Sync` users will struggle to use your library in `async` or multy-
        /// threaded applications. Using `Box<dyn>` allows us to hold a pointer to any type of
        /// error that implements these traits.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    }

    // We either write the `message` by itself, or we write it along with the underlying error's
    // message.
    impl Display for BetterError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self.source.as_ref() {
                None => Display::fmt(&self.message, f),
                Some(source) => write!(f, "{}: {}", self.message, source),
            }
        }
    }

    impl Error for BetterError {
        /// The default trait implementation of this function simply returns `None`. Here we are
        /// overriding it so that our error "plays nice" and returns any underlying error. Through
        /// this mechanism, a chain of wrapping errors is built up that can serve as a pseudo-
        /// stack-trace.
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            // This is ugly and took by a long time to figure out, but we are just casting our
            // `source` to the type that is defined by the `Error` trait.
            self.source.as_ref().map(|e| e.as_ref() as &(dyn Error))
        }
    }
}

/// # The Question Mark Operator (p.160)
/// What is the `?` operator actually doing?
mod question_mark_operator {}

/// # The Question Mark Operator Calls Into (p.162)
/// The `?` operator can convert errors if they implement `From`.
/// For brevity the errors in this example are not "proper errors".
mod question_mark_into_call {
    struct ErrorTypeOne;
    struct ErrorTypeTwo;

    impl From<ErrorTypeOne> for ErrorTypeTwo {
        fn from(e: ErrorTypeOne) -> Self {
            // pretend there is some real transformation here
            Self
        }
    }

    fn returns_error_type_one() -> Result<(), ErrorTypeOne> {
        Err(ErrorTypeOne)
    }

    fn returns_error_type_two() -> Result<(), ErrorTypeTwo> {
        // the conversion to ErrorTypeTwo is automatic when ? is used
        Ok(returns_error_type_one()?)
    }

    struct ErrorTypeThree;

    // This will not compile because ErrorTypeOne cannot be automatically converted to
    // ErrorTypeThree
    // fn returns_error_type_three() -> Result<(), ErrorTypeThree> {
    //     Ok(returns_error_type_one()?)
    // }
}

/// # Errors and the `main` Function (p. 164)
/// The main function will show an error using `Debug`, which is too bad.
/// This is why we usually catch the error from a `run` function and display it with `eprintln!()`.
fn main() {
    use mane::*;
    println!("Chapter 9!");
}

mod mane {}

/// # Custom Types for Validation (p. 167)
/// In Rust it is idiomatic to prevent a function from accepting bad input by making it impossible
/// with the type system.
mod type_system_validation {}

// TODO - # Demo Snafu

// TODO - # Demo Anyhow

// TODO - # Demo thiserror

// TODO - # Demo Eyre
