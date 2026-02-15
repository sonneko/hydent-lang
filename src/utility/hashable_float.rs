//! Float type that can be hashed.

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HashableFloat<T>(T)
where
    T: std::fmt::Debug + Clone + Copy + PartialEq + PartialOrd;

impl<T> HashableFloat<T>
where
    T: std::fmt::Debug + Clone + Copy + PartialEq + PartialOrd,
{
    pub fn new(value: T) -> Self {
        HashableFloat(value)
    }

    pub fn get(&self) -> T {
        self.0
    }
}

impl std::hash::Hash for HashableFloat<f32> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.get() == 0.0 {
            0.0f32.to_bits().hash(state)
        } else if self.get().is_nan() {
            f32::NAN.to_bits().hash(state)
        } else {
            f32::to_bits(self.get()).hash(state)
        }
    }
}

impl std::hash::Hash for HashableFloat<f64> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.get() == 0.0 {
            0.0f64.to_bits().hash(state)
        } else if self.get().is_nan() {
            f64::NAN.to_bits().hash(state)
        } else {
            f64::to_bits(self.get()).hash(state)
        }
    }
}

impl<T> Eq for HashableFloat<T> where T: std::fmt::Debug + Clone + Copy + PartialEq + PartialOrd {}

impl<T> From<T> for HashableFloat<T>
where
    T: std::fmt::Debug + Clone + Copy + PartialEq + PartialOrd,
{
    fn from(value: T) -> Self {
        HashableFloat(value)
    }
}

impl<T> std::fmt::Display for HashableFloat<T>
where
    T: std::fmt::Debug + Clone + Copy + PartialEq + PartialOrd,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HashableFloat({:?})", self.0)
    }
}
