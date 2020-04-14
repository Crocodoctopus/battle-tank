#[derive(Debug, Copy, Clone)]
pub struct Vec2<T>(pub T, pub T);
#[derive(Debug, Copy, Clone)]
pub struct Vec3<T>(pub T, pub T, pub T);
#[derive(Debug, Copy, Clone)]
pub struct Vec4<T>(pub T, pub T, pub T, pub T);

pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec4f = Vec4<f32>;

pub type Vec2i = Vec2<i32>;
pub type Vec3i = Vec3<i32>;
pub type Vec4i = Vec4<i32>;

pub type Vec2u = Vec2<u32>;
pub type Vec3u = Vec3<u32>;
pub type Vec4u = Vec4<u32>;

impl<T> Vec2<T> {
    pub fn tuple(self) -> (T, T) {
        (self.0, self.1)
    }
}
impl<T> std::ops::Add for Vec2<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
impl<T> std::ops::Sub for Vec2<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}
impl<T> std::ops::Mul for Vec2<T>
where
    T: std::ops::Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1)
    }
}

impl<T> Vec3<T> {
    fn tuple(self) -> (T, T, T) {
        (self.0, self.1, self.2)
    }
}
impl<T> std::ops::Add for Vec3<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl<T> std::ops::Sub for Vec3<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl<T> std::ops::Mul for Vec3<T>
where
    T: std::ops::Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl<T> Vec4<T> {
    fn tuple(self) -> (T, T, T, T) {
        (self.0, self.1, self.2, self.3)
    }
}
impl<T> std::ops::Add for Vec4<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}
impl<T> std::ops::Sub for Vec4<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}
impl<T> std::ops::Mul for Vec4<T>
where
    T: std::ops::Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
        )
    }
}
