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
