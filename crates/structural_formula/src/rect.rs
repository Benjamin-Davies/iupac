use glam::Vec2;

use crate::structure::Structure;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    /// The identity of the union operation.
    pub const UNION_IDENTITY: Rect = Rect {
        min: Vec2::INFINITY,
        max: Vec2::NEG_INFINITY,
    };

    pub fn from_points(points: impl IntoIterator<Item = Vec2>) -> Self {
        points
            .into_iter()
            .fold(Self::UNION_IDENTITY, |rect, point| rect.expand(point))
    }

    pub fn expand(self, point: Vec2) -> Self {
        Self {
            min: self.min.min(point),
            max: self.max.max(point),
        }
    }

    pub fn width(self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(self) -> f32 {
        self.max.y - self.min.y
    }
}

impl Structure {
    pub fn bounds(&self) -> Rect {
        Rect::from_points(self.graph.node_weights().map(|atom| atom.position))
    }
}
