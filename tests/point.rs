extern crate rsmath as r;

#[cfg(test)]
mod tests {
    use r::linspace::point::*;

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
}
