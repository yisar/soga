use crate::red;
use crate::green;

pub struct FlexBox {
    pub records: Vec<FlexItem>,
    pub prev_pos: isize,
    pos1: usize,
    pos2: usize,
    size1: usize,
    size2: usize,
}
#[derive(Clone, Debug)]
pub struct FlexItem {
    pub rect: [isize; 4],
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
        let mut max_h: isize = 0;
        let mut max_w: isize = 0;
        let p_width = item.width();
        let p_height = item.height();
        let direction = item.direction();
        let wrap = item.wrap();

        let mut flex_dim: isize = 0;

        let mut grows = item.grows();
        let mut shrinks = item.shrinks();

        match direction {
            green::Direction::Row => {
                flex_dim = item.width() as isize;
                self.pos1 = 0;
                self.pos2 = 1;
                self.size1 = 2;
                self.size2 = 3;
            }
            green::Direction::Column => {
                flex_dim = item.height() as isize;
                self.pos1 = 1;
                self.pos2 = 0;
                self.size1 = 3;
                self.size2 = 2;
            }
        }

        for child in item.children() {
            let mut temp_rect = [0; 4];

            temp_rect[self.size1] = child.width();
            temp_rect[self.size2] = child.height();

            flex_dim -= temp_rect[self.size1] as isize;

            grows += child.grow();
            shrinks += child.shrink();
        }

        for child in item.children() {
            let child_width = child.width();
            let child_height = child.height();

            let mut flexitem = FlexItem {
                rect: [0, 0, 0, 0],
            };

            flexitem.rect[self.size1] = child_width;
            flexitem.rect[self.size2] = child_height;
            flexitem.rect[self.pos1] = x;

            let mut size: isize = 0;

            match wrap {
                green::Wrap::Wrap => {
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
                green::Wrap::NoWrap => {
                    if flex_dim > 0 {
                        if child.grow() != 0 {
                            size += (flex_dim / grows) * child.grow();
                        }
                    } else if flex_dim < 0 {
                        if child.shrink() != 0 {
                            size += (flex_dim / shrinks) * child.shrink();
                        }
                    }

                }
            }

            flexitem.rect[self.size1] += size;

            x += flexitem.rect[self.size1];
            y += flexitem.rect[self.size2];

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