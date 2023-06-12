pub trait Scalar: std::ops::Add<Self, Output = Self>
                 + std::ops::Sub<Self, Output = Self>
                 + std::ops::Mul<Self, Output = Self>
                 + std::ops::Div<Self, Output = Self>
where
    Self: Sized,
{
    type Item;
    
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = u32;
    
    fn zero() -> Self::Item {
        0
    }
    
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for u64 {
    type Item = u64;
    
    fn zero() -> Self::Item {
        0
    }
    
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i32 {
    type Item = i32;
    
    fn zero() -> Self::Item {
        0
    }
    
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i64 {
    type Item = i64;
    
    fn zero() -> Self::Item {
        0
    }
    
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for f32 {
    type Item = f32;
    
    fn zero() -> Self::Item {
        0.0
    }
    
    fn one() -> Self::Item {
         1.0
    }
}

impl Scalar for f64 {
    type Item = f64;
    
    fn zero() -> Self::Item {
        0.0
    }
    
    fn one() -> Self::Item {
         1.0
    }
}
