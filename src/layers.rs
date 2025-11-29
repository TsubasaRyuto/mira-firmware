pub const LAYER0: usize = 0;
pub const LAYER1: usize = 1;
pub const LAYER2: usize = 2;
pub const LAYER3: usize = 3;
pub const LAYER4: usize = 4;
pub const LAYER5: usize = 5;
pub const LAYER6: usize = 6;
pub const LAYER7: usize = 7;
pub const LAYER8: usize = 8;

pub struct Layer {
    pub active_layer: usize
}

impl Layer {
    pub fn new() -> Self {
        Self {
            active_layer: LAYER0
        }
    }

    pub fn update(&mut self, layer: usize) {
        self.active_layer = layer;
    }
}
