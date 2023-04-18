

pub fn flatten<I>(iter:I) -> Flatten<I> where I:Iterator, I::Item:IntoIterator{
    Flatten::new(iter)
}
pub struct Flatten<O> where O:Iterator {
    outer:O,
    inner:Option<<O::Item as IntoIterator>::IntoIter>
}
impl<O> Flatten<O> where O:Iterator, O::Item:IntoIterator{
    fn new(iter:O)->Self{
        Flatten{outer:iter, inner:None}
    }
}
impl<O> Iterator for Flatten<O>
where
    O:Iterator,
    O::Item:IntoIterator
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self)->Option<Self::Item>{
        if let Some(ref mut inner_iter) = self.inner{
            if let Some(i) = inner_iter.next(){
                return Some(i)
            }
            self.inner = None;
        }
        let next_inner_iter = self.outer.next()?.into_iter();
        // self.inner = Some(next_inner_iter)

    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn empty(){
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(),0)

    }
}