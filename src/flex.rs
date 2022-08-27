use crate::red;
use crate::green;

pub struct FlexBox {
    pub records: Vec<FlexItem>,
    pub prev_pos: usize,
}
#[derive(Clone, Debug)]
pub struct FlexItem {
    pub rect: [usize; 4],
}

impl FlexBox {
    pub fn new() -> FlexBox {
        FlexBox {
            records: Vec::new(),
            prev_pos: 0,
        }
    }
    pub fn layout(&mut self, item: red::RedTree) {
        let mut x = 0;
        let mut y = 0;
        let mut max_h = 0;
        let mut max_w = 0;
        let p_width = item.width();
        let p_height = item.height();
        let direction = item.direction();
        let wrap = item.wrap();

        for child in item.children() {
            let child_width = child.width();
            let child_height = child.height();

            let mut flexitem = FlexItem {
                rect: [0, 0, child_width, child_width],
            };

            match direction {
                green::Direction::Row => {
                    flexitem.rect[0] = x;
                }
                green::Direction::Column => {
                    flexitem.rect[1] = y;
                }
            }

            x += child_width;
            y += child_height;

            if wrap == green::Wrap::Wrap {
                if x > p_width && direction == green::Direction::Row {
                    let ret = self.prev_pos + max_h;
                    flexitem.rect[0] = 0;
                    flexitem.rect[1] = ret;
                    self.prev_pos = ret;
                    max_h = 0;
                }
                if y > p_height && direction == green::Direction::Column {
                    let ret = self.prev_pos + max_w;
                    flexitem.rect[1] = 0;
                    flexitem.rect[0] = ret;
                    self.prev_pos = ret;
                    max_w = 0;
                }
            }

            if child_width > max_w {
                max_w = child_width;
            }

            if child_height > max_h {
                max_h = child_height;
            }

            self.records.push(flexitem);
            self.layout(child);
        }
    }
}