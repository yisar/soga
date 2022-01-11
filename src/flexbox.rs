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
    children:  Vec<Box<FlexItem>>,
    frame: (i64, i64, f64, f64),
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
    pub fn flex_item_with_size(&self, width: f64, height: f64) -> FlexItem {
        FlexItem {
            width,
            height,
            direction: Direction::Row,
            children: vec![],
            frame: (0, 0, 0.0, 0.0),
            grow: 0,
            shrink: 0,
        }
    }

    pub fn flex_item_new(&self) -> FlexItem {
        FlexItem {
            width: 0.0,
            height: 0.0,
            direction: Direction::Row,
            children: vec![],
            frame: (0, 0, 0.0, 0.0),
            grow: 0,
            shrink: 0,
        }
    }

    pub fn flex_item_set_width(&mut self, mut child: FlexItem, width: f64) {
        child.width = width;
    }

    pub fn flex_item_set_height(&mut self, mut child: FlexItem, height: f64) {
        child.height = height;
    }

    pub fn flex_layout(&mut self, mut root: &FlexItem) {
        self.layout_item(root, root.width, root.height)
    }

    fn layout_item(&mut self, mut item: &FlexItem, width: f64, height: f64) {
        self.layout_init(item, width, height);

        for i in item.children.iter() {
            // i.frame = (0, 0, i.width, i.height);
            self.flex_grows += i.grow;
            self.flex_shrinks += i.shrink;
            self.flex_dim -= i.frame.0
        }
    }

    fn layout_init(&mut self, item: &FlexItem, width: f64, height: f64) {
        self.reverse = false;
        self.vertical = true;

        match item.direction {
            Direction::Row => {
                self.vertical = false;
                self.size_dim = width;
                self.align_dim = height;
                self.frame_pos = 0;
                self.frame_pos2 = 1;
                self.frame_size = 2;
                self.frame_size2 = 3;
            }
            Direction::RowReverse => self.reverse = true,
            Direction::Column => {
                self.vertical = false;
                self.size_dim = width;
                self.align_dim = height;
                self.frame_pos = 1;
                self.frame_pos2 = 0;
                self.frame_size = 3;
                self.frame_size2 = 2;
            }
            Direction::ColumnReverse => self.reverse = true,
        }

        self.flex_dim = 0;
        self.flex_grows = 0;
        self.flex_shrinks = 0;
    }
}

impl FlexItem {
    pub fn flex_item_add(&mut self, child: FlexItem) {
        self.children.push(Box::new(child))
    }

    pub fn flex_item_delete(&mut self, index: usize) {
        self.children.remove(index);
        ()
    }
}