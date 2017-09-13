

use std::iter::Fuse;

pub struct ClampToExactLength<I, F> {
    pub(crate) iter: Fuse<I>,
    pub(crate) filler: F,
    pub(crate) index: usize,
    pub(crate) size: usize,
}

impl<I, F> Iterator for ClampToExactLength<I, F>
    where I: Iterator,
          F: FnMut(usize) -> I::Item,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.size {
            None
        } else {
            let index = self.index;
            self.index += 1;
            match self.iter.next() {
                None => Some((self.filler)(index)),
                elt => elt,
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.size - self.index;
        (len, Some(len))
    }
}


pub struct AssertExactSize<I> {
    pub(crate) iter: I,
    pub(crate) size: usize,
}

impl<I> AssertExactSize<I>
    where I: Iterator,
{
    fn get_next<F>(&mut self, next: F) -> Option<I::Item>
        where F: FnOnce(&mut I) -> Option<I::Item>
    {
        let elt = next(&mut self.iter);
        if let Some(_) = elt {
            debug_assert!(self.size > 0, "AssertExactSize: length mismatch, iterator\
                produced at least one element more than the asserted size");
            self.size -= 1;
        } else {
            debug_assert_eq!(0, self.size, "AssertExactSize: length mismatch, iterator\
                produced {} elements less than the asserted size", self.size);
        }
        elt
    }
}

impl<I> Iterator for AssertExactSize<I> where I: Iterator
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.get_next(|iter| iter.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<I> ExactSizeIterator for AssertExactSize<I> where I: Iterator { }
impl<I> DoubleEndedIterator for AssertExactSize<I>
    where I: DoubleEndedIterator
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.get_next(|iter| iter.next_back())
    }

}
