use crate::red;
use crate::green;

pub struct FlexBox {
    pub records: Vec<FlexItem>,
    pub prev_pos: usize,
    pos1: usize,
    pos2: usize,
    size1: usize,
    size2: usize,
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
            pos1: 0,
            pos2: 0,
            size1: 0,
            size2: 0,
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

        let mut grows = item.grows();
        let mut shrinks = item.shrinks();

        match direction {
            green::Direction::Row => {
                self.pos1 = 0;
                self.pos2 = 1;
                self.size1 = 2;
                self.size2 = 3;
            }
            green::Direction::Column => {
                self.pos1 = 1;
                self.pos2 = 0;
                self.size1 = 3;
                self.size2 = 2;
            }
        }

        for child in item.children() {
            // flex_dim -= child.frame[self.size1] as isize;

            // println!("{:#?}", child.grow());

            grows += child.grow();
            shrinks += child.shrink();
        }

        println!("{:#?}", grows);

        for child in item.children() {
            let child_width = child.width();
            let child_height = child.height();

            let mut flexitem = FlexItem {
                rect: [0, 0, 0, 0],
            };

            flexitem.rect[self.size1] = child_width;
            flexitem.rect[self.size2] = child_height;

            x += child_width;
            y += child_height;

            if wrap == green::Wrap::Wrap {
                if x > p_width && direction == green::Direction::Row {
                    let ret = self.prev_pos + max_h;
                    flexitem.rect[self.pos1] = 0;
                    flexitem.rect[self.pos2] = ret;
                    self.prev_pos = ret;
                    max_h = 0;
                }
                if y > p_height && direction == green::Direction::Column {
                    let ret = self.prev_pos + max_w;
                    flexitem.rect[self.pos2] = 0;
                    flexitem.rect[self.pos1] = ret;
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