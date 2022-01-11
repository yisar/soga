#[derive(Clone, Debug)]
pub struct FlexBox {
    reverse: bool,
    vertical: bool,
    grows: i64,
    shrinks: i64,
}
#[derive(Clone, Debug)]
pub struct FlexItem {
    width: f64,
    height: f64,
    direction: Direction,
    children: Vec<Box<FlexItem>>,
    frame: (f64, f64, f64, f64),
    grow: i64,
    shrink: i64,
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
        }
    }

    pub fn layout(&mut self, item: &mut FlexItem) {
        let mut x = 0.0;
        let mut y = 0.0;

        let mut dim = item.height;

        for child in item.children.iter_mut() {
            child.frame.0 = x;
            child.frame.1 = y;
            child.frame.2 = child.width;
            child.frame.3 = child.height;

            x += child.width;
            y += child.height;

            if child.grow == 0 {
                dim -= child.height;
            }

            self.grows += child.grow;
            self.shrinks += child.shrink;
        }

        for child in item.children.iter_mut() {
            if child.grow != 0 {
                child.frame.3 = (dim / (self.grows as f64)) * child.grow as f64;
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

    pub fn new(width: f64, height: f64) -> FlexItem {
        FlexItem {
            width,
            height,
            direction: Direction::Row,
            children: vec![],
            frame: (0.0, 0.0, 0.0, 0.0),
            grow: 0,
            shrink: 0,
        }
    }

    pub fn default() -> FlexItem {
        FlexItem {
            width: 0.0,
            height: 0.0,
            direction: Direction::Row,
            children: vec![],
            frame: (0.0, 0.0, 0.0, 0.0),
            grow: 0,
            shrink: 0,
        }
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    pub fn set_grow(&mut self, grow: i64) {
        self.grow = grow;
    }
}
