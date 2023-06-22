#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn length(&self, other: &Vector2) -> f32 {
        let x = (self.x - other.x).abs();
        let y = (self.y - other.y).abs();
        (x.powi(2) + y.powi(2)).sqrt()
    }

    pub fn gte(&self, other: &Vector2) -> bool {
        self.x >= other.x && self.y >= other.y
    }

    pub fn lte(&self, other: &Vector2) -> bool {
        self.x <= other.x && self.y <= other.y
    }
}

#[cfg(test)]
mod test {
    use super::Vector2;

    #[test]
    fn ord() {
        let l = Vector2 { x: 0.0, y: 0.0 };
        let b = Vector2 { x: 1.0, y: 0.0 };
        assert!(l.lte(&b));
        assert!(b.gte(&l));
        let l = Vector2 { x: 0.0, y: 0.0 };
        let b = Vector2 { x: 1.0, y: 1.0 };
        assert!(l.lte(&b));
        assert!(b.gte(&l));
        let l = Vector2 { x: 0.0, y: 0.0 };
        let b = Vector2 { x: 1.0, y: -1.0 };
        assert!(!l.lte(&b));
        assert!(!b.gte(&l));
        let l = Vector2 { x: 0.0, y: 0.0 };
        let b = Vector2 { x: -1.0, y: 10.0 };
        assert!(!l.lte(&b));
        assert!(!b.gte(&l));
    }
}
