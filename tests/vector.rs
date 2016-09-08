extern crate ralg as r;

#[cfg(test)]
mod tests {
    use r::calcul::vector::*;

    #[test]
    fn vector_init_test() {
        let v3 = Vector3D::<f64>::init();

        assert_eq!(v3.x(), 0.0);
        assert_eq!(v3.y(), 0.0);
        assert_eq!(v3.z(), 0.0);
    }

    #[test]
    fn vector_init_with_values_test() {
        let v3 = Vector3D::<f64>::init_with_values(1.0, 2.5, -3.0);

        assert_eq!(v3.x(), 1.0);
        assert_eq!(v3.y(), 2.5);
        assert_eq!(v3.z(), -3.0);
    }
    #[test]
    fn vector_scalar_mul_test() {
        let v3 = Vector3D::<i32>::init_with_values(-1, 2, 3);
        let sm = v3.scalar_mul(2);

        assert_eq!(sm.x(), v3.x() * 2);
        assert_eq!(sm.y(), v3.y() * 2);
        assert_eq!(sm.z(), v3.z() * 2);
    }
    #[test]
    fn vector_add_test() {
        let a = Vector3D::<i32>::init_with_values(-2, 3, 5);
        let b = Vector3D::<i32>::init_with_values(5, -2, -3);

        let sum = a + b;

        assert_eq!(sum.x(), a.x() + b.x());
        assert_eq!(sum.y(), a.y() + b.y());
        assert_eq!(sum.z(), a.z() + b.z());
    }
    #[test]
    fn vector_sub_test() {
        let a = Vector3D::<f32>::init_with_values(0.2, 10.1, -2.5);
        let b = Vector3D::<f32>::init_with_values(5.7, -2.1, -3.0);

        let sub = a - b;

        assert_eq!(sub.x(), a.x() - b.x());
        assert_eq!(sub.y(), a.y() - b.y());
        assert_eq!(sub.z(), a.z() - b.z());
    }
    #[test]
    fn vector_mul_test() {
        let a = Vector3D::<u32>::init_with_values(0, 10, 5);
        let b = Vector3D::<u32>::init_with_values(7, 2, 0);

        let mul = a * b;

        assert_eq!(mul.x(), a.x() * b.x());
        assert_eq!(mul.y(), a.y() * b.y());
        assert_eq!(mul.z(), a.z() * b.z());
    }
}
