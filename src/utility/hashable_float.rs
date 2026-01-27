
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HashableFloat<T>(T)
where
    T: std::fmt::Debug + Clone + Copy + PartialEq + PartialOrd;

impl<T> HashableFloat<T>
where
    T: std::fmt::Debug + Clone + Copy + PartialEq + PartialOrd + ToBits
{
    pub fn new(value: T) -> Self {
        HashableFloat(value)
    }
    
    pub fn get(&self) -> T {
        self.0
    }
}

impl<T> std::hash::Hash for HashableFloat<T>
where 
    T: std::fmt::Debug + Clone + Copy + PartialEq + PartialOrd + ToBits
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let float = self.0;
        float.to_bits().hash(state)
    }
}

impl<T> Eq for HashableFloat<T>
where 
    T: std::fmt::Debug + Clone + Copy + PartialEq + PartialOrd + ToBits
{}

impl<T> From<T> for HashableFloat<T>
where 
    T: std::fmt::Debug + Clone + Copy + PartialEq + PartialOrd + ToBits
{
    fn from(value: T) -> Self {
        HashableFloat(value)
    }
}

impl<T> std::fmt::Display for HashableFloat<T>
where 
    T: std::fmt::Debug + Clone + Copy + PartialEq + PartialOrd + ToBits
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HashableFloat({:?})", self.0)
    }
}

pub trait ToBits {
    type Bits: std::hash::Hash;
    fn to_bits(&self) -> Self::Bits;
}

impl ToBits for f32 {
    type Bits = u32;
    fn to_bits(&self) -> u32 { f32::to_bits(*self) }
}

impl ToBits for f64 {
    type Bits = u64;
    fn to_bits(&self) -> u64 { f64::to_bits(*self) }
}
