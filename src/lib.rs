//! A macro for generating a merged struct from multiple sub-structs.
//!
//! # Example
//!
//! ```
//! use multi_structs::multi_structs;
//!
//! multi_structs! {
//!     /// The merged struct.
//!     #[derive(Debug)]
//!     struct Merged {
//!         /// Foo
//!         #[derive(Debug)]
//!         foo: struct Foo {
//!             /// a
//!             a: i32,
//!             /// b
//!             b: i64,
//!         }
//!         /// Bar
//!         #[derive(Debug)]
//!         bar: struct Bar {
//!             /// c
//!             c: usize,
//!             /// d
//!             d: String,
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let foo = Foo { a: 1, b: 2 };
//!     let bar = Bar { c: 3, d: "aaa".to_string() };
//!     println!("{:?}, {:?}", foo, bar);
//!     let merged = Merged::new(foo, bar);
//!     println!("{:?}", merged);
//!     let (foo, bar) = merged.split();
//!     println!("{:?}, {:?}", foo, bar);
//! }
//! ```
//!
//! See [`example_generated`](./example_generated/index.html) for
//! documentation of code generated by the above `multi_structs!`
//! expansion.
//!
//! # Attributes
//!
//! Attributes can be attached to any struct and field involved.
//!
//! # Methods
//!
//! The following methods are defined for the merged struct:
//!
//! - `new`: create the new merged struct from multiple sub-structs.
//! - `split`: split the merged struct into the sub-structs.
//!
//! # Visibility
//!
//! The visibility of structs and fields is taken directly from their definitions. For the
//! generated methods `new` and `split`, the visibility of the merged struct is assumed.

#![no_std]

#[cfg(feature = "example_generated")]
extern crate std;

#[macro_export]
macro_rules! multi_structs {
    {
        $(#[$($meta:tt)+])*
        $multi_vis:vis struct $name:ident {
            $(
                $(#[$($sub_meta:tt)+])*
                $sub_vis:vis $var:ident: struct $sub:ident {
                    $(
                        $(#[$($field_meta:tt)+])*
                        $field_vis:vis $field:ident: $ty:ty
                    ),+ $(,)?
                }
            )+
        }
    } => {
        $(
            $(#[$($sub_meta)+])*
            $sub_vis struct $sub {
                $(
                    $(#[$($field_meta)+])*
                    $field_vis $field: $ty,
                )+
            }
        )+

        $(#[$($meta)+])*
        $multi_vis struct $name {
            $(
                $(
                    $(#[$($field_meta)+])*
                    $field_vis $field: $ty,
                )+
            )+
        }

        impl $name {
            /// Create this struct from sub-structs.
            $multi_vis fn new(
                $($var: $sub,)+
            ) -> Self {
                Self {
                    $(
                        $($field: $var.$field,)+
                    )+
                }
            }

            /// Split this struct into its sub-structs.
            $multi_vis fn split(self) -> ($($sub,)+) {
                (
                    $(
                        $sub {
                            $($field: self.$field,)+
                        },
                    )+
                )
            }
        }
    }
}

#[cfg(feature = "example_generated")]
pub mod example_generated;
