use rand::Rng;
use smallvec::SmallVec;

pub const VEC_SIZE: usize = 4;

pub trait Swap {
    fn swap(self) -> Self;
}

impl<T> Swap for (T, T) {
    fn swap(self) -> Self {
        (self.1, self.0)
    }
}

pub fn conditional_swap<T>(pair: (T, T), should_swap: bool) -> (T, T) {
    if should_swap {
        pair.swap()
    } else {
        pair
    }
}

pub fn normalize_vec(vec: &mut SmallVec<[f32; VEC_SIZE]>) {
    let mut sum = 0.0;
    let size = vec.len();

    for value in &mut *vec {
        sum += *value;
    }

    for value in vec {
        if sum > 0.0 {
            *value /= sum;
        } else {
            *value = 1.0 / (size as f32);
        }
    }
}

// Pick a random number using a probability distribution vector thingy
pub fn roulette<R>(probabilities: &[f32], rng: &mut R) -> usize
where
    R: Rng,
{
    let upper = 100000;
    let num: f32 = rng.gen_range(0..upper) as f32 / (upper as f32);
    let mut total = 0.0;

    for (index, length) in probabilities.into_iter().enumerate() {
        if num >= total && num < total + length {
            return index;
        }

        total += *length;
    }

    panic!("Value {:?} fit nowhere", num)
}
