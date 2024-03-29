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
        let direction = item.direction();
        let wrap = item.wrap();

        let mut flex_dim: isize = 0;
        let mut align_dim: isize = 0;
        let mut size_dim: isize = 0;

        let mut grows = 0;
        let mut shrinks = 1;

        match direction {
            green::Direction::Row => {
                flex_dim = item.width() as isize;
                size_dim = item.width() as isize;
                align_dim = item.height() as isize;
                self.pos1 = 0;
                self.pos2 = 1;
                self.size1 = 2;
                self.size2 = 3;
            }
            green::Direction::Column => {
                flex_dim = item.height() as isize;
                size_dim = item.height() as isize;
                align_dim = item.width() as isize;
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
                rect: [0; 4],
            };

            flexitem.rect[self.size1] = child_width;
            flexitem.rect[self.size2] = child_height;

            let mut size: isize = 0;

            match wrap {
                green::Wrap::Wrap => {
                    let child_size = flexitem.rect[self.size1];
                    if child_size > size_dim  {
                        x = 0;
                        y += flexitem.rect[self.size2];
                    } else {
                        size_dim -= child_size;
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

            if x == 0 {
                match item.justify_content() {
                    green::Align::Center => {
                        x = flex_dim / 2;
                    }
                    green::Align::Auto  => {}
                }
            }

            flexitem.rect[self.pos1] += x;

            let mut align = y;

            match item.align_items() {
                green::Align::Center => {
                    align = (align_dim / 2) - (flexitem.rect[self.size2] / 2);
                }
                green::Align::Auto => {}
            }

            flexitem.rect[self.size1] += size;
            flexitem.rect[self.pos2] = align;

            x += flexitem.rect[self.size1];
            y += flexitem.rect[self.size2];

            self.records.push(flexitem);
            self.layout(child);
        }
    }
}
