use num::Num;

pub struct Vector3D<N: Copy> {
    x: N,
    y: N,
    z: N,
}

impl<N: Copy> Vector3D<N> {
    pub fn init() -> Vector3D<N> where N: Num + Default {
        Vector3D {x: N::default(), y: N::default(), z: N::default()}
    }

    pub fn x(&self) -> N {
        self.x
    }

    pub fn y(&self) -> N {
        self.y
    }
    pub fn z(&self) -> N {
        self.z
    }
}
