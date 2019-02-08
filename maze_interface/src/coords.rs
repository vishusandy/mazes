/// This structure holds the two x and two y coordinates for a rectage.  These
/// are not (x, y) coordinates but rather the west and east x coordinates, and
/// the north and south y coordinates.
///
/// The fields of this struct are public because certain methods impls require
/// Copy and specialization has not yet landed to allow a Copy and Clone impl.
pub struct RectPoints<T: num::ToPrimitive> {
    pub x_w: T,
    pub x_e: T,
    pub y_n: T,
    pub y_s: T,
}

impl<T: num::ToPrimitive + Copy> RectPoints<T> {
    pub fn new(x_w: T, x_e: T, y_n: T, y_s: T) -> Self {
        unimplemented!() // TODO
    }
    pub fn x_w(&self) -> T {
        self.x_w
    }
    pub fn x_e(&self) -> T {
        self.x_e
    }
    pub fn y_n(&self) -> T {
        self.y_n
    }
    pub fn y_s(&self) -> T {
        self.y_s
    }
}

/// This structure represents four (x, y) coordinates of a given type (a number).
///
/// The fields of this struct are public because certain methods impls require
/// Copy and specialization has not yet landed to allow a Copy and Clone impl.
pub struct RectCoords<T: num::ToPrimitive> {
    pub nw: (T, T),
    pub ne: (T, T),
    pub se: (T, T),
    pub sw: (T, T),
}

impl<T: num::ToPrimitive + Copy> RectCoords<T> {
    pub fn nw(&self) -> (T, T) {
        self.nw
    }
    pub fn ne(&self) -> (T, T) {
        self.ne
    }
    pub fn se(&self) -> (T, T) {
        self.se
    }
    pub fn sw(&self) -> (T, T) {
        self.sw
    }
}
impl<T: num::ToPrimitive> RectCoords<T> {
    pub fn new(nw: (T, T), ne: (T, T), se: (T, T), sw: (T, T)) -> Self {
        Self { nw, ne, se, sw }
    }
    pub fn as_float(&self) -> Option<RectCoords<f32>> {
        let rect: RectCoords<f32> = RectCoords {
            nw: (self.nw.0.to_f32()?, self.nw.1.to_f32()?),
            ne: (self.ne.0.to_f32()?, self.ne.1.to_f32()?),
            se: (self.se.0.to_f32()?, self.se.1.to_f32()?),
            sw: (self.sw.0.to_f32()?, self.sw.1.to_f32()?),
        };
        Some(rect)
    }
    pub fn as_u32(&self) -> Option<RectCoords<u32>> {
        let rect: RectCoords<u32> = RectCoords {
            nw: (self.nw.0.to_u32()?, self.nw.1.to_u32()?),
            ne: (self.ne.0.to_u32()?, self.ne.1.to_u32()?),
            se: (self.se.0.to_u32()?, self.se.1.to_u32()?),
            sw: (self.sw.0.to_u32()?, self.sw.1.to_u32()?),
        };
        Some(rect)
    }
    pub fn as_u16(&self) -> Option<RectCoords<u16>> {
        let rect: RectCoords<u16> = RectCoords {
            nw: (self.nw.0.to_u16()?, self.nw.1.to_u16()?),
            ne: (self.ne.0.to_u16()?, self.ne.1.to_u16()?),
            se: (self.se.0.to_u16()?, self.se.1.to_u16()?),
            sw: (self.sw.0.to_u16()?, self.sw.1.to_u16()?),
        };
        Some(rect)
    }
}
