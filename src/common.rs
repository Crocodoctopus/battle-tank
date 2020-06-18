#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn vec2f(self) -> Vec2f {
        match self {
            Direction::Up => Vec2(0.0, -1.0),
            Direction::Down => Vec2(0.0, 1.0),
            Direction::Left => Vec2(-1.0, 0.0),
            Direction::Right => Vec2(1.0, 0.0),
        }
    }
}

pub fn clamp<T: PartialOrd>(low: T, t: T, high: T) -> T {
    if t < low {
        low
    } else if t > high {
        high
    } else {
        t
    }
}

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
impl<T> std::ops::Add<Self> for Vec2<T>
where
    T: std::ops::Add<T, Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
impl<T> std::ops::Sub<Self> for Vec2<T>
where
    T: std::ops::Sub<T, Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}
impl<T> std::ops::Mul<Self> for Vec2<T>
where
    T: std::ops::Mul<T, Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1)
    }
}
impl<T> std::ops::Mul<f32> for Vec2<T>
where
    T: std::ops::Mul<f32, Output = T>,
{
    type Output = Self;
    fn mul(self, other: f32) -> Self::Output {
        Self(self.0 * other, self.1 * other)
    }
}

impl<T> Vec3<T> {
    fn tuple(self) -> (T, T, T) {
        (self.0, self.1, self.2)
    }
}
impl<T> std::ops::Add<Self> for Vec3<T>
where
    T: std::ops::Add<T, Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
impl<T> std::ops::Sub<Self> for Vec3<T>
where
    T: std::ops::Sub<T, Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
impl<T> std::ops::Mul<Self> for Vec3<T>
where
    T: std::ops::Mul<T, Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl<T> Vec4<T> {
    fn tuple(self) -> (T, T, T, T) {
        (self.0, self.1, self.2, self.3)
    }
}
impl<T> std::ops::Add<Self> for Vec4<T>
where
    T: std::ops::Add<T, Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}
impl<T> std::ops::Sub<Self> for Vec4<T>
where
    T: std::ops::Sub<T, Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}
impl<T> std::ops::Mul<Self> for Vec4<T>
where
    T: std::ops::Mul<T, Output = T>,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
        )
    }
}
