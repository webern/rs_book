/*!

# Chapter 6: Enums

pp. 97-110 in the 2018 edition paperback.

Rust enums are inspired by OCaml and Haskell!
They are *algebraic data types*.

 */
#![allow(dead_code, unused_variables, unused_mut)]

fn main() {
    println!("Chapter 6!");
}

/// Enums with and Without Data
mod enums_with_and_without_data {
    /// Plain old Enum
    enum PlainEnumNoData {
        Red,
        Green,
        Blue,
    }

    /// A simple Struct
    struct Point {
        x: u64,
        y: u64,
    }

    /// An enum that can carry various types of data
    enum DataEnum {
        /// This variant does not have any data
        Empty,

        /// This variant carries a named struct. Notice that it is tempting to use the same name
        /// for the data struct and the variant. This is fine but it can get annoying if auto import
        /// functionality gets confused by it.
        Point(Point),

        /// This variant has an unnamed struct. Personally I prefer to use a named struct.
        UnnamedStruct { x: u64, y: u64 },
    }

    fn example() {
        let value_1 = DataEnum::Empty;
        let value_2 = DataEnum::Point(Point { x: 0, y: 1 });
        let value_3 = DataEnum::UnnamedStruct { x: 0, y: 1 };

        match value_3 {
            DataEnum::Empty => todo!(),
            DataEnum::Point(Point { x: 0, .. }) => todo!(),
            DataEnum::UnnamedStruct { x: 0, .. } => todo!(),
            _ => todo!(),
        }
    }
}
