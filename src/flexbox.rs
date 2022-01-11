#[derive(Clone, Debug)]
pub struct FlexBox {
    reverse: bool,
    vertical: bool,
    dim: usize,
    size_i: usize,
    pos_i: usize,
    grows: usize,
    shrinks: usize,
}
#[derive(Clone, Debug)]
pub struct FlexItem {
    width: usize,
    height: usize,
    direction: Direction,
    children: Vec<Box<FlexItem>>,
    frame: Vec<usize>,
    grow: usize,
    shrink: usize,
}
#[derive(Clone, Debug)]
enum Direction {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl FlexBox {
    pub fn new() -> FlexBox {
        FlexBox {
            reverse: false,
            vertical: false,
            grows: 0,
            shrinks: 0,
            pos_i: 0,
            dim: 0,
            size_i: 0,
        }
    }

    pub fn layout(&mut self, item: &mut FlexItem) {
        match &item.direction {
            Direction::Row => {
                self.dim = item.width;
                self.pos_i = 0;
                self.size_i = 2;
            }
            Direction::RowReverse => {
                self.reverse = true;
            }
            Direction::Column => {
                self.dim = item.height;
                self.pos_i = 1;
                self.size_i = 3;
            }
            Direction::ColumnReverse => self.reverse = false,
        }

        let mut pos = if self.reverse { self.dim } else { 0 };
        let mut dim = item.height;
        let mut size = 0;

        for child in item.children.iter_mut() {
            child.frame[0] = 0;
            child.frame[1] = 0;
            child.frame[2] = child.width;
            child.frame[3] = child.height;

            dim -= child.frame[self.size_i];

            self.grows += child.grow;
            self.shrinks += child.shrink;
        }

        for child in item.children.iter_mut() {
            if dim > 0 {
                if child.grow != 0 {
                    size += (dim / (self.grows as usize)) * child.grow as usize;
                }
            } else if dim < 0 {
                if child.shrink != 0 {
                    size += (dim / (self.shrinks as usize)) * (child.shrink as usize);
                }
            }

            child.frame[self.size_i] += size;

            if self.reverse {
                pos -= child.frame[self.size_i];
                child.frame[self.pos_i] = pos;
            } else {
                child.frame[self.pos_i] = pos;
                pos += child.frame[self.size_i];
            }
        }
    }

    pub fn flex_layout(&mut self, root: &mut FlexItem) {
        self.layout(root)
    }
}

impl FlexItem {
    pub fn add(&mut self, child: FlexItem) {
        self.children.push(Box::new(child))
    }

    pub fn delete(&mut self, index: usize) {
        self.children.remove(index);
        ()
    }

    pub fn new(width: usize, height: usize) -> FlexItem {
        FlexItem {
            width,
            height,
            direction: Direction::Row,
            children: vec![],
            frame: vec![0, 0, 0, 0],
            grow: 0,
            shrink: 0,
        }
    }

    pub fn default() -> FlexItem {
        FlexItem {
            width: 0,
            height: 0,
            direction: Direction::Row,
            children: vec![],
            frame: vec![0, 0, 0, 0],
            grow: 0,
            shrink: 0,
        }
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
    }

    pub fn set_grow(&mut self, grow: usize) {
        self.grow = grow;
    }
}
