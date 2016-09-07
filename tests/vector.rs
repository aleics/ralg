extern crate ralg as r;

#[cfg(test)]
mod tests {
    use r::calcul::vector::*;

    fn init_test() {
        let v3 = Vector3D::<f64>::init();

        assert_eq!(v3.x(), 0.0);
        assert_eq!(v3.y(), 0.0);
        assert_eq!(v3.z(), 0.0);
    }
}
