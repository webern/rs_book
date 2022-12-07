#![allow(dead_code, unused_variables, unused_mut)]

use crate::some_trait::ClashingTrait;

/// From p 117, "child" code can use private code from its "parent" but not the other way around
mod a {
    fn private() {}

    // This is not allowed. The "parent" cannot use the "child's" private symbols.
    // fn nope() {
    //     crate::a::b::private();
    // }

    mod b {
        /// This is allowed. The "child" can use the "parent's" private symbols.
        fn private() {
            crate::a::private()
        }
    }
}

/// p 123 Idiomatic function namespacing
pub mod utils {
    pub mod frobincators {
        pub fn frobnicate() {}
    }
}

pub mod do_this {
    use crate::utils::frobincators;

    fn some_function() {
        frobincators::frobnicate()
    }
}

pub mod not_this {
    use crate::utils::frobincators::frobnicate;

    fn some_function() {
        frobnicate()
    }
}

pub mod some_module {
    pub struct ClashingName;
}

/// Renaming imports
mod rename_a_function {
    use crate::some_module::ClashingName as Renamed;

    pub struct ClashingName;

    fn some_function() {
        let _ = Renamed;
    }
}

pub mod some_trait {
    pub trait ClashingTrait {
        fn foo(&self);
    }
}

struct Foo;

impl ClashingTrait for Foo {
    fn foo(&self) {
        println!("foo");
    }
}

trait FooExt {
    fn cooler_stuff(&self);
}

impl<T> ClashingTrait for Option<T> {
    fn foo(&self) {
        if self.is_none() {
            println!("none")
        }
    }
}

mod rename_a_trait {
    use crate::Foo;

    // sometimes you just need to bring a trait into a scope and don't care about the name. If the
    // name clashes with something, you can do this.
    use crate::some_trait::ClashingTrait as _;

    trait ClashingTrait {}

    fn use_foo() {
        let x = Foo;
        x.foo();
    }
}

pub mod x {
    pub use y::X;

    pub mod y {
        pub struct X {}
    }

    #[cfg(test)]
    mod testing {
        use super::*;

        #[test]
        fn test() {
            X
        }
    }
}

fn dfkjgh() {
    x::X {};
    // x::y::X {};
}
