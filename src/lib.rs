//! Itertools-wild — extra wild iterator adaptors, wild functions and wild macros.
//!
//! To use the iterator methods in this crate, import the [`ItertoolsWild` trait](./trait.ItertoolsWild.html):
//!
//! ```ignore
//! use itertools_wild::ItertoolsWild;
//! ```
//!
//! ## Rust Version
//!
//! This version of itertools-wild requires wild Rust
//!
#![doc(html_root_url="https://docs.rs/itertools-wild/")]

extern crate either;
extern crate itertools;

pub mod adaptors;
pub mod special;

use adaptors::{
    ClampToExactSize,
    AssertExactSize,
};

pub trait ItertoolsWild : Iterator {
    fn assert_exact_size(self, size: usize) -> AssertExactSize<Self>
        where Self: Sized,
    {
        AssertExactSize {
            iter: self,
            size: size,
        }
    }

    fn clamp_to_exact_size<F>(self, size: usize, filler: F) -> ClampToExactSize<Self, F>
        where Self: Sized,
              F: FnMut(usize) -> Self::Item,
    {
        ClampToExactSize {
            iter: self.fuse(),
            index: 0,
            size: size,
            filler: filler,
        }
    }

}


impl<I> ItertoolsWild for I where I: Iterator { }
