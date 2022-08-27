use crate::red;

pub struct FlexBox {
    pub records: Vec<FlexItem>,
}
#[derive(Clone, Debug)]
pub struct FlexItem {
    pub rect: [usize; 4],
}

impl FlexBox {
    pub fn new() -> FlexBox {
        FlexBox {
            records: vec![],
        }
    }
    pub fn layout(&mut self, item: red::RedTree) {
        let mut pos = 0;
        let mut size = 0;

        for child in item.children() {
            let flexitem = FlexItem {
                rect: [pos, 0, child.width(), child.height()],
            };

            self.records.push(flexitem);
            pos += child.width();
            self.layout(child);
        }
    }
}