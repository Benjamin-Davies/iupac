use glam::Affine2;

use crate::structure::{Atom, Structure};

impl Structure {
    pub fn transform(&mut self, matrix: Affine2) {
        for atom in self.graph.node_weights_mut() {
            atom.transform(matrix);
        }
    }
}

impl Atom {
    pub fn transform(&mut self, matrix: Affine2) {
        self.position = matrix.transform_point2(self.position);
    }
}
