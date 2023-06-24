#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn length(&self, other: Option<&Vector2>) -> f32 {
        let (x, y) = if let Some(other) = other {
            ((self.x - other.x).abs(), (self.y - other.y).abs())
        } else {
            (self.x, self.y)
        };

        (x.powi(2) + y.powi(2)).sqrt()
    }

    pub fn gte(&self, other: &Vector2) -> bool {
        self.x >= other.x && self.y >= other.y
    }

    pub fn lte(&self, other: &Vector2) -> bool {
        self.x <= other.x && self.y <= other.y
    }

    pub fn move_once(&mut self, Vector2 { x, y }: &Vector2) {
        self.x += 0.02 * x;
        self.y += 0.02 * y;
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
