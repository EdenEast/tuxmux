use std::iter::Fuse;

pub struct Intersperse<I>
where
    I: Iterator,
{
    element: I::Item,
    iter: Fuse<I>,
    peek: Option<I::Item>,
}

impl<I> Iterator for Intersperse<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;
    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        if self.peek.is_some() {
            self.peek.take()
        } else {
            self.peek = self.iter.next();
            if self.peek.is_some() {
                Some(self.element.clone())
            } else {
                None
            }
        }
    }
}

/// Create a new Intersperse iterator
pub fn intersperse<I>(iter: I, elt: I::Item) -> Intersperse<I>
where
    I: Iterator,
{
    let mut iter = iter.fuse();
    Intersperse {
        peek: iter.next(),
        iter,
        element: elt,
    }
}
