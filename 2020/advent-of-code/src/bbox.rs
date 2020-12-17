
use num_traits::Num;

#[derive(Copy, Clone)]
pub struct BBox3D<T>(T,T,T,T,T,T);// xmin, xmax, ymin, ymax, zmin, zmax
impl<T> BBox3D<T> where T: Num + std::cmp::PartialOrd + Copy {
    pub fn new() -> BBox3D<T> {
        BBox3D(T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero())
    }

    #[inline]
    pub fn xmin(&self) -> T { self.0 }
    #[inline]
    pub fn xmax(&self) -> T { self.1 }
    #[inline]
    pub fn ymin(&self) -> T { self.2 }
    #[inline]
    pub fn ymax(&self) -> T { self.3 }
    #[inline]
    pub fn zmin(&self) -> T { self.4 }
    #[inline]
    pub fn zmax(&self) -> T { self.5 }

    #[inline]
    pub fn update(&mut self, (x, y, z): &(T, T, T)) {
        if *x < self.0 { self.0 = *x; }
        else if *x > self.1 { self.1 = *x; }
        if *y < self.2 { self.2 = *y; }
        else if *y > self.3 { self.3 = *y; }
        if *z < self.4 { self.4 = *z; }
        else if *z > self.5 { self.5 = *z; }
    }
}
impl<T> std::fmt::Display for BBox3D<T> where T: std::fmt::Display + Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BBox: {}, {}, {}, {}, {}, {}", self.0, self.1, self.2, self.3, self.4, self.5)
    }
}


#[derive(Copy, Clone)]
pub struct BBox4D<T>(T,T,T,T,T,T,T,T);// xmin, xmax, ymin, ymax, zmin, zmax
impl<T> BBox4D<T> where T: Num + std::cmp::PartialOrd + Copy {
    pub fn new() -> BBox4D<T> {
        BBox4D(T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero(),T::zero())
    }

    #[inline]
    pub fn xmin(&self) -> T { self.0 }
    #[inline]
    pub fn xmax(&self) -> T { self.1 }
    #[inline]
    pub fn ymin(&self) -> T { self.2 }
    #[inline]
    pub fn ymax(&self) -> T { self.3 }
    #[inline]
    pub fn zmin(&self) -> T { self.4 }
    #[inline]
    pub fn zmax(&self) -> T { self.5 }
    #[inline]
    pub fn wmin(&self) -> T { self.6 }
    #[inline]
    pub fn wmax(&self) -> T { self.7 }

    #[inline]
    pub fn update(&mut self, (x, y, z, w): &(T, T, T, T)) {
        if *x < self.0 { self.0 = *x; }
        else if *x > self.1 { self.1 = *x; }
        if *y < self.2 { self.2 = *y; }
        else if *y > self.3 { self.3 = *y; }
        if *z < self.4 { self.4 = *z; }
        else if *z > self.5 { self.5 = *z; }
        if *w < self.6 { self.6 = *w; }
        else if *w > self.7 { self.7 = *w; }
    }
}
impl<T> std::fmt::Display for BBox4D<T> where T: std::fmt::Display + Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BBox: {}, {}, {}, {}, {}, {}, {}, {}", self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7)
    }
}
