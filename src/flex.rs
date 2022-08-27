use crate::red;
use crate::green;

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
        let direction = item.direction();
        let wrap = item.wrap();

        println!("{:#?}", wrap);

        for child in item.children() {
            let mut flexitem = FlexItem {
                rect: [0, 0, child.width(), child.height()],
            };

            match direction {
                green::Direction::Row =>{
                    flexitem.rect[0] = pos;
                }
                green::Direction::Column=>{
                    flexitem.rect[1] = size;
                }
            }

            self.records.push(flexitem);

            pos += child.width();
            size += child.height();
            self.layout(child);
        }
    }
}