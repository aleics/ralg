extern crate ralg as r;

#[cfg(test)]
mod tests {
    use r::linspace::point::*;

    #[test]
    fn point_init_test() {
        let v3 = Point3D::<f64>::init();

        assert_eq!(v3.x(), 0.0);
        assert_eq!(v3.y(), 0.0);
        assert_eq!(v3.z(), 0.0);
    }

    #[test]
    fn point_init_with_values_test() {
        let v3 = Point3D::<f64>::init_with_values(1.0, 2.5, -3.0);

        assert_eq!(v3.x(), 1.0);
        assert_eq!(v3.y(), 2.5);
        assert_eq!(v3.z(), -3.0);
    }
    #[test]
    fn point_scale_test() {
        let mut v3 = Point3D::<i32>::init_with_values(-1, 2, 3);
        let copy = v3.clone();
        v3.scale(2);

        assert_eq!(v3.x(), copy.x() * 2);
        assert_eq!(v3.y(), copy.y() * 2);
        assert_eq!(v3.z(), copy.z() * 2);
    }
    #[test]
    fn point_add_test() {
        let a = Point3D::<i32>::init_with_values(-2, 3, 5);
        let b = Point3D::<i32>::init_with_values(5, -2, -3);

        let sum = a + b;

        assert_eq!(sum.x(), a.x() + b.x());
        assert_eq!(sum.y(), a.y() + b.y());
        assert_eq!(sum.z(), a.z() + b.z());
    }
    #[test]
    fn point_sub_test() {
        let a = Point3D::<f32>::init_with_values(0.2, 10.1, -2.5);
        let b = Point3D::<f32>::init_with_values(5.7, -2.1, -3.0);

        let sub = a - b;

        assert_eq!(sub.x(), a.x() - b.x());
        assert_eq!(sub.y(), a.y() - b.y());
        assert_eq!(sub.z(), a.z() - b.z());
    }
    #[test]
    fn point_mul_test() {
        let a = Point3D::<u32>::init_with_values(0, 10, 5);
        let b = Point3D::<u32>::init_with_values(7, 2, 0);

        let mul = a * b;

        assert_eq!(mul.x(), a.x() * b.x());
        assert_eq!(mul.y(), a.y() * b.y());
        assert_eq!(mul.z(), a.z() * b.z());
    }
    #[test]
    fn point_set_x_test(){
        let mut a = Point3D::<u64>::init_with_values(256, 450, 178);
        a.set_x(200);

        assert_eq!(a.x(), 200);
        assert_eq!(a.y(), a.y());
        assert_eq!(a.z(), a.z());
    }
    #[test]
    fn point_set_y_test(){
        let mut a = Point3D::<i64>::init_with_values(-25, 40, -17);
        a.set_y(20);

        assert_eq!(a.x(), a.x());
        assert_eq!(a.y(), 20);
        assert_eq!(a.z(), a.z());
    }
    #[test]
    fn point_set_z_test(){
        let mut a = Point3D::<f32>::init_with_values(25.6, 4.5, 17.8);
        a.set_z(2.5);

        assert_eq!(a.x(), a.x());
        assert_eq!(a.y(), a.y());
        assert_eq!(a.z(), 2.5);
    }
    #[test]
    fn point_set_test(){
        let mut a = Point3D::<f32>::init_with_values(25.6, 4.5, 17.8);
        a.set(2.5, 0.1, 1.2);

        assert_eq!(a.x(), 2.5);
        assert_eq!(a.y(), 0.1);
        assert_eq!(a.z(), 1.2);
    }
}
