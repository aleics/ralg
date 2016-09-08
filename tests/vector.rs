extern crate ralg as r;

#[cfg(test)]
mod tests {
    use r::linspace::vector::*;
    use r::linspace::point::*;

    #[test]
    fn vector_init_test() {
        let v = Vector3D::<f64>::init();

        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 0.0);
        assert_eq!(v.z(), 0.0);
        assert_eq!(v.get_origin() == Point3D::<f64>::init(), true);
    }
}
