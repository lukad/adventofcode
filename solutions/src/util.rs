/// Iterator extension trait for Least Common Multiple (LCM).
pub trait LcmExt: Iterator {
    fn lcm<A>(self) -> A
    where
        A: Lcm<Self::Item>,
        Self: Sized,
    {
        A::lcm(self)
    }
}

impl<I: Iterator> LcmExt for I {}

pub trait Lcm<A = Self> {
    fn lcm<I>(iter: I) -> Self
    where
        I: Iterator<Item = A>;
}

impl<T> Lcm for T
where
    T: num::Integer,
{
    fn lcm<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::one(), |a, b| a.lcm(&b))
    }
}

impl<'a, T> Lcm<&'a T> for T
where
    T: num::Integer,
{
    fn lcm<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::one(), |a, b| a.lcm(b))
    }
}

pub trait SignedModulo {
    #[allow(unused)]
    fn modulo(&self, n: Self) -> Self;
}

macro_rules! impl_signed_modulo {
    ($($t:ty)*) => ($(
      impl SignedModulo for $t {
            fn modulo(&self, n: Self) -> Self {
                let r = self % n;
                if r < 0 {
                    r + n
                } else {
                    r
                }
            }
      }
    )*)
}

impl_signed_modulo! { i8 i16 i32 i64 }
