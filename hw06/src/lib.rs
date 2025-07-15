pub struct Integer(u32);

pub struct Float(f32);

pub struct Tuple(i32, i32);

pub struct Array([i32; 3]);

impl Integer {
    pub fn double_as_u32(&self) -> u32 {
        self.0 * 2
    }

    pub fn double_as_u64(&self) -> u64 {
        (self.0 * 2) as u64
    }

    pub fn add_float_to_float(&self, b: f32) -> f64 {
        self.0 as f64 + b as f64
    }

    pub fn add_float_to_int(&self, b: f32) -> u64 {
        self.0 as u64 + b as u64
    }
}

impl Float {
    pub fn double_as_float32(&self) -> f32 {
        self.0 * 2.0
    }

    pub fn double_as_float64(&self) -> f64 {
        (self.0 * 2.0) as f64
    }
}

impl Tuple {
    pub fn sum(&self) -> i32 {
        self.0 + self.1
    }
}

impl Array {
    pub fn sum(&self) -> i32 {
        self.0.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_as_u32() {
        let i = Integer(8);
        let result = i.double_as_u32();
        assert_eq!(result, 16);
    }

    #[test]
    fn test_double_as_u64() {
        let i = Integer(8);
        let result = i.double_as_u64();
        assert_eq!(result, 16);
    }

    #[test]
    fn test_double_as_float32() {
        let i = Float(8.0);
        let result = i.double_as_float32();
        assert_eq!(result, 16.0);
    }

    #[test]
    fn test_double_as_float64() {
        let i = Float(8.0);
        let result = i.double_as_float64();
        assert_eq!(result, 16.0);
    }

    #[test]
    fn test_add_float_to_float() {
        let i = Integer(8);
        let j = 2.0;
        let result = i.add_float_to_float(j);
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_add_float_to_int() {
        let i = Integer(8);
        let j = 2.0;
        let result = i.add_float_to_int(j);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_tuple_sum() {
        let t = Tuple(1, 2);
        let result = t.sum();
        assert_eq!(result, 3)
    }

    #[test]
    fn test_array_sum() {
        let a = Array([2, 4, 6]);
        let result = a.sum();
        assert_eq!(result, 12)
    }
}
