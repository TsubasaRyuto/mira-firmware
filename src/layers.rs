pub const LAYER0: u32 = 0;
pub const LAYER1: u32 = 1;
pub const LAYER2: u32 = 2;
pub const LAYER3: u32 = 3;
pub const LAYER4: u32 = 4;
pub const LAYER5: u32 = 5;
pub const LAYER6: u32 = 6;
pub const LAYER7: u32 = 7;
pub const LAYER8: u32 = 8;

pub struct Layer {
    pub state: u32,
    pub active_layer: u32,
}

impl Layer {
    pub fn new() -> Self {
        Self {
            state: 0b0000_0001,
            active_layer: LAYER0,
        }
    }

    pub fn press(&mut self, layer: u32) {
        self.state |= (1 << layer);
        self.set_active_layer();
    }

    pub fn release(&mut self, layer: u32) {
        self.state &= !(1 << layer);
        self.set_active_layer();
    }

    fn set_active_layer(&mut self) {
        self.active_layer = 31 - self.state.leading_zeros();
    }
}
