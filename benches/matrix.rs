extern crate test;
extern crate ralg as r;

#[cfg(test)]
mod tests {

    use r::algebra::matrix::*;
    use test::Bencher;

    #[bench]
    fn create_random_bench(b: &mut Bencher) {
        let range: [f32; 2] = [0.0, 5.0];
        b.iter(|| Matrix::<f32>::create_random(3, 3, &range));
    }
}
