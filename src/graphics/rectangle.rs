/// A generic rectangle.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Rectangle<T> {
    /// X coordinate of the top-left corner.
    pub x: T,

    /// Y coordinate of the top-left corner.
    pub y: T,

    /// Width of the rectangle.
    pub width: T,

    /// Height of the rectangle.
    pub height: T,
}
