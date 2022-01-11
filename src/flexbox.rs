#[derive(Clone, Debug)]
pub struct FlexBox {
    reverse: bool,
    vertical: bool,
    size_dim: f64,
    align_dim: f64,
    frame_pos: i64,
    frame_pos2: i64,
    frame_size: i64,
    frame_size2: i64,
    flex_dim: i64,
    flex_grows: i64,
    flex_shrinks: i64,
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
            size_dim: 0.0,
            align_dim: 0.0,
            frame_pos: 0,
            frame_pos2: 0,
            frame_size: 0,
            frame_size2: 0,
            flex_dim: 0,
            flex_grows: 0,
            flex_shrinks: 0,
        }
    }

    pub fn layout(&mut self, item: &mut FlexItem) {
        let mut x = 0.0;
        let mut y = 0.0;
        for i in item.children.iter_mut() {
            i.frame.0 = x;
            i.frame.1 = y;
            x += i.width;
            y += i.height;
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
}
