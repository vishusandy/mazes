pub struct RectCoords<T: num::ToPrimitive> {
    nw: (T, T),
    ne: (T, T),
    se: (T, T),
    sw: (T, T),
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
