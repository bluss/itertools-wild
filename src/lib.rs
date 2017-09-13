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
    ClampToExactLength,
    AssertExactSize,
};

pub trait ItertoolsWild : Iterator {
    /// Assert that the iterator has an exact size hint; the result
    /// is an iterator that produces the same sequence as usual but also
    /// implements `ExactSizeIterator`
    ///
    /// Note that this is not anything like `unsafe`, since it's not related
    /// to memory safety -- only logical consistency.
    ///
    /// The `AssertExactSize` adaptor will use **debug assertions** to check
    /// the iterator's actual length.
    fn assert_exact_size(self, size: usize) -> AssertExactSize<Self>
        where Self: Sized,
    {
        AssertExactSize {
            iter: self,
            size: size,
        }
    }

    /// Clamp an iterator to an exact length
    ///
    /// The adapted iterator never produces more than the `length` amount
    /// of elements. If the underlying iterator would end short of that,
    /// the closure `filler` is called to supply elements up until the
    /// specific `length`.
    ///
    /// This iterator can be trusted to have the exact length, and it is fused.
    fn clamp_to_exact_length<F>(self, length: usize, filler: F) -> ClampToExactLength<Self, F>
        where Self: Sized,
              F: FnMut(usize) -> Self::Item,
    {
        ClampToExactLength {
            iter: self.fuse(),
            index: 0,
            size: length,
            filler: filler,
        }
    }

}


impl<I> ItertoolsWild for I where I: Iterator { }
