use std::{ops::{Add, Sub, Mul, AddAssign, SubAssign, Div, Rem}, fmt::Debug};

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

impl<T> FiniteFieldElement<T>
where
    T: Rem<Output = T> + Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd + Copy + DefaultNumber + Debug
{
    fn pow(self, e: T) -> Self {
        let one = T::one();
        let mut r = Self::new(one, self.p);
        let mut i = e % (self.p - one);
        let zero = T::zero();
        while i > zero {
            r = r * self;
            i = i - one;
        }
        r
    }
}

impl<T> Add for FiniteFieldElement<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq + PartialOrd + Copy
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.p != rhs.p {
            panic!("Cannot add different field value.")
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
            panic!("Cannot sub different field value.");
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
            panic!("Cannot mul different field value.");
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

impl<T> Div for FiniteFieldElement<T>
where
    T: Add<Output = T> + Sub<Output = T> + Rem<Output = T> + PartialEq + PartialOrd + Copy + DefaultNumber + Debug
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let one = T::one();
        self * rhs.pow(self.p - one - one)
    }
}
