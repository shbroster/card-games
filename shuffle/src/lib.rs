use std::collections::VecDeque;
use rand;

trait LenAndSwap {
    fn len(&self) -> usize;
    fn swap(&mut self, i: usize, j: usize);
}

/// An exact copy of rand::Rng::shuffle with a modified signature
///
/// The new function allows any type that implements LenAndSwap;
/// it is implemented primarily to allow shuffling og VecDeque
fn shuffle<T, R>(values: &mut T, mut rng: R)
where
    T: LenAndSwap,
    R: rand::Rng,
{
    let mut i = values.len();
    while i >= 2 {
        i -= 1;
        // Lock the elements >i in place and swap the ith element
        // with a random element <i. 
        values.swap(i, rng.gen_range(0..i + 1));
    }
}

/// Implement LenAndSwap for VecDeque so that we can use shuffle
impl<T> LenAndSwap for VecDeque<T> {
    fn len(&self) -> usize {
        self.len()
    }
    fn swap(&mut self, i: usize, j: usize) {
        self.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn vecdeque_can_use_shuffle() {
        let mut rng = rand::thread_rng();
        let mut vd: VecDeque<i64> = (1..64).collect();
        shuffle(&mut vd, rng);
    }
}
