
use std::cell::Cell;

mod inner {
    use std::cell::Cell;

    pub struct Inner<'a, T: 'a>(pub &'a Cell<Option<T>>);

    impl<'a, T: Copy> Iterator for Inner<'a, T> {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            self.0.get()
        }
    }
}

/// Self-referential iterator adaptor builder
///
/// See `feedback` for more information.
pub struct Feedback<T> {
    t: Cell<Option<T>>,
}

/// Self-referential iterator adaptor
///
/// See `feedback` for more information.
pub struct FeedbackIter<'a, I, T: 'a> {
    iter: I,
    inner: &'a Cell<Option<T>>,
}

/**
 * Feed the output of an iterator pipeline back to the input. Feedback
 * is delayed by one timestep to preserve causality, so the first input
 * is provided by `initial`, and the output of that pass is used as the
 * input for the second pass, and so on.
 *
 * Every time the input is requested, it yields the last result returned
 * by the pipeline. The pipeline can request the feedback value any
 * number of times per cycle, including ignoring it entirely. If the
 * pipeline doesn't request a particular input, that input is discarded,
 * not saved for the next cycle. If the pipeline requests an input
 * multiple times in the process of producing an output, the same input
 * will be returned each time.
 *
 * ```rust
 * use itertools_wild::special::feedback;
 *
 * let input = [1, -2, 3, -4, 5];
 * let result: Vec<i32> = feedback(0).feed(|feedback|
 *         feedback.zip(&input)
 *                 .map(|(a, b)| a + b)
 *     ).collect();
 * assert_eq!(result, &[0, 1, -1, 2, -2, 3]);
 * ```
 */
pub fn feedback<T>(initial: T) -> Feedback<T>
    where T: Copy
{
    Feedback {
        t: Cell::new(Some(initial)),
    }
}


impl<T> Feedback<T> where T: Copy
{
    pub fn feed<'a, F, I>(&'a mut self, inner: F) -> FeedbackIter<'a, I, T>
        where F: FnOnce(inner::Inner<'a, T>) -> I,
              T: Copy
    {
        FeedbackIter {
            iter: inner(inner::Inner(&self.t)),
            inner: &self.t,
        }
    }
}

impl<'a, I, T> Iterator for FeedbackIter<'a, I, T>
    where I: Iterator<Item = T>,
          T: Copy
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Some(v) = self.inner.get() {
            self.inner.set(self.iter.next());
            Some(v)
        } else {
            None
        }
    }
}
