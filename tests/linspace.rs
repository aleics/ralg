extern crate rsmath as r;

#[cfg(test)]
mod tests {
    use r::linspace::vector::*;
    use r::linspace::point::*;

    // --------------- Point3D TEST ----------------------------------------

    #[test]
    fn point_init_test() {
        let p3 = Point3D::<f64>::init();

        assert_eq!(p3.x(), 0.0);
        assert_eq!(p3.y(), 0.0);
        assert_eq!(p3.z(), 0.0);
    }

    #[test]
    fn point_init_with_values_test() {
        let p3 = Point3D::<f64>::init_with_values(1.0, 2.5, -3.0);

        assert_eq!(p3.x(), 1.0);
        assert_eq!(p3.y(), 2.5);
        assert_eq!(p3.z(), -3.0);
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
    #[test]
    fn point_eucl_distance_test() {
        let p1 = Point3D::<i32>::init_with_values(-1, 2, 3);
        let p2 = Point3D::<i32>::init_with_values(0, 5, 8);

        let d = Point3D::<i32>::eucl_distance(&p1, &p2);

        assert_eq!(d, 5.916079783099616); // using online euclidean distance calculator
    }

    // --------------- Vector3D TEST ----------------------------------------

    #[test]
    fn vector_init_test() {
        let v = Vector3D::<f64>::init();

        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 0.0);
        assert_eq!(v.z(), 0.0);
    }
    #[test]
    fn vector_init_with_values_test() {
        let v = Vector3D::<f64>::init_with_values(2.0, 3.1, 5.2);

        assert_eq!(v.x(), 2.0);
        assert_eq!(v.y(), 3.1);
        assert_eq!(v.z(), 5.2);
    }
    #[test]
    fn vector_scale_test() {
        let mut v3 = Vector3D::<i32>::init_with_values(-1, 2, 3);
        let copy = v3.clone();
        v3.scale(2);

        assert_eq!(v3.x(), copy.x() * 2);
        assert_eq!(v3.y(), copy.y() * 2);
        assert_eq!(v3.z(), copy.z() * 2);
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
    #[test]
    fn vector_set_x_test(){
        let mut a = Vector3D::<u64>::init_with_values(256, 450, 178);
        a.set_x(200);

        assert_eq!(a.x(), 200);
        assert_eq!(a.y(), a.y());
        assert_eq!(a.z(), a.z());
    }
    #[test]
    fn vector_set_y_test(){
        let mut a = Vector3D::<i64>::init_with_values(-25, 40, -17);
        a.set_y(20);

        assert_eq!(a.x(), a.x());
        assert_eq!(a.y(), 20);
        assert_eq!(a.z(), a.z());
    }
    #[test]
    fn vector_set_z_test(){
        let mut a = Vector3D::<f32>::init_with_values(25.6, 4.5, 17.8);
        a.set_z(2.5);

        assert_eq!(a.x(), a.x());
        assert_eq!(a.y(), a.y());
        assert_eq!(a.z(), 2.5);
    }
    #[test]
    fn vector_set_test(){
        let mut a = Vector3D::<f32>::init_with_values(25.6, 4.5, 17.8);
        a.set(2.5, 0.1, 1.2);

        assert_eq!(a.x(), 2.5);
        assert_eq!(a.y(), 0.1);
        assert_eq!(a.z(), 1.2);
    }
    #[test]
    fn vector_cross_test() {
        let a = Vector3D::<i32>::init_with_values(-1, 2, 3);
        let b = Vector3D::<i32>::init_with_values(0, 2, 5);

        let c = Vector3D::<i32>::cross(&a, &b);

        let res = Vector3D::<i32>::init_with_values(4, 5, -2); // online cross vector calculator

        assert_eq!(res == c, true);
    }
    #[test]
    fn vector_dot_test() {
        let a = Vector3D::<i64>::init_with_values(1, 0, 2);
        let b = Vector3D::<i64>::init_with_values(0, 2, 1);

        assert_eq!(a.dot(&b), 1*0 + 0*2 + 2*1);
    }
    #[test]
    fn vector_norm_test() {
        let mut a = Vector3D::<f64>::init_with_values(2f64, 1f64, 3f64);
        a.norm();

        // values compared using wolframalpha.com
        assert_eq!(a.x(), 0.5345224838248488f64);
        assert_eq!(a.y(), 0.2672612419124244f64);
        assert_eq!(a.z(), 0.8017837257372732f64);
    }
}
