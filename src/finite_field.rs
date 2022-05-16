use std::ops::{Add, Sub, Mul, AddAssign, SubAssign};

pub trait DefaultNumber {
    fn zero() -> Self;
    fn one() -> Self;
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct FiniteFieldElement<T>
where
    T: PartialEq + Copy
{
    pub value: T,
    pub p: T
}

impl<T> FiniteFieldElement<T>
where
    T: PartialEq + Copy
{
    pub fn new(value: T, p: T) -> Self {
        Self { value, p }
    }
}

impl<T> Add for FiniteFieldElement<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd + Copy
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.p != rhs.p {
            panic!("Different field.")
        }
        let sum = self.value + rhs.value;
        if sum >= self.p {
            Self::new(sum - self.p, self.p)
        } else {
            Self::new(sum, self.p)
        }
    }
}

impl<T> AddAssign for FiniteFieldElement<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd + Copy
{
    fn add_assign(&mut self, rhs: Self) {
        *self = (*self) + rhs
    }
}

impl<T> Sub for FiniteFieldElement<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd + Copy
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.p != rhs.p {
            panic!("Different field.");
        }
        if self.value < rhs.value {
            Self::new(self.p - rhs.value + self.value , self.p)
        } else {
            Self::new(self.value - rhs.value, self.p)
        }
    }
}

impl<T> SubAssign for FiniteFieldElement<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd + Copy
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = (*self) - rhs
    }
}

impl<T> Mul for FiniteFieldElement<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd + Copy + DefaultNumber
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.p != rhs.p {
            panic!("Different field.");
        }
        let one = T::one();
        let zero = T::zero();
        let mut i = rhs.value;
        let mut r = Self::new(zero, self.p);
        while i > zero {
            r += self;
            i = i - one;
        }
        r
    }
}
