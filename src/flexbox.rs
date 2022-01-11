#[derive(Clone, Debug)]
pub struct FlexBox {
    reverse: bool,
    vertical: bool,
    size_i: usize,
    pos_i: usize,
    size2_i: usize,
    pos2_i: usize,
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
    align_self: Align,
    align_items: Align,
    basis: usize,
    justify_content: Align,
}
#[derive(Clone, Debug)]
pub enum Direction {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Align {
    Auto,
    Center,
    FlexStart,
    FlexEnd,
    Stretch,
    Between,
    Around,
}

impl FlexBox {
    pub fn new() -> FlexBox {
        FlexBox {
            reverse: false,
            vertical: false,
            grows: 0,
            shrinks: 0,
            pos_i: 0,
            size_i: 0,
            size2_i: 0,
            pos2_i: 0,
        }
    }

    pub fn layout(&mut self, item: &mut FlexItem) {
        let mut flex_dim = 0;
        let mut align_dim = 0;
        let mut size = 0;

        match &item.direction {
            Direction::Row => {
                flex_dim = item.width;
                align_dim = item.height;
                self.pos_i = 0;
                self.pos2_i = 1;
                self.size_i = 2;
                self.size2_i = 3;
            }
            Direction::RowReverse => {
                self.reverse = true;
            }
            Direction::Column => {
                flex_dim = item.height;
                align_dim = item.width;
                self.pos_i = 1;
                self.pos2_i = 0;
                self.size_i = 3;
                self.size2_i = 2;
            }
            Direction::ColumnReverse => self.reverse = false,
        }

        let mut pos = if self.reverse { flex_dim } else { 0 };

        for child in item.children.iter_mut() {
            child.frame[0] = 0;
            child.frame[1] = 0;
            child.frame[2] = child.width;
            child.frame[3] = child.height;

            flex_dim -= child.frame[self.size_i];

            self.grows += child.grow;
            self.shrinks += child.shrink;

            if child.basis > 0 {
                child.frame[self.size_i] = child.basis;
            }
        }

        for child in item.children.iter_mut() {
            if flex_dim > 0 {
                if child.grow != 0 {
                    size = (flex_dim / (self.grows as usize)) * child.grow as usize;
                }
            } else {
                if child.shrink != 0 {
                    size = (flex_dim / (self.shrinks as usize)) * (child.shrink as usize);
                }
            }

            child.frame[self.size_i] += size;
            let mut spacing = 0;
            match child.justify_content {
                Align::FlexEnd => {
                    pos = flex_dim;
                }
                Align::Center => {
                    pos = flex_dim / 2;
                }
                Align::Between => {
                    if child.children.len() > 0 {
                        spacing = flex_dim / (child.children.len() - 1);
                    }
                }
                Align::Around => {
                    if child.children.len() > 0 {
                        spacing = flex_dim / child.children.len();
                        pos = spacing / 2;
                    }
                }
                _ => {}
            }

            let a_pos = child.frame[self.size_i] + spacing;

            if self.reverse {
                pos -= a_pos;
                child.frame[self.pos_i] = pos;
            } else {
                child.frame[self.pos_i] = pos;
                pos += a_pos;
            }

            let mut align = 0;
            let mut align_type = &child.align_self;

            if align_type == &Align::Auto {
                align_type = &child.align_items;
            }
            match align_type {
                Align::Auto => {}
                Align::FlexStart => {}
                Align::Center => {
                    align = (align_dim / 2) - (child.frame[self.size2_i] / 2);
                }
                Align::FlexEnd => {
                    align = align_dim - child.frame[self.size2_i];
                }
                Align::Stretch => {
                    align = 0;
                    child.frame[self.size2_i] = align_dim;
                }
                _ => {}
            }
            child.frame[self.pos2_i] = align;
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
            basis: 0,
            align_self: Align::Auto,
            align_items: Align::Auto,
            justify_content: Align::Auto,
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
            basis: 0,
            align_self: Align::Auto,
            align_items: Align::Auto,
            justify_content: Align::Auto,
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

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn set_align_self(&mut self, align: &str) {
        let align_type = match align {
            "center" => Align::Center,
            "flex-start" => Align::FlexStart,
            "flex-end" => Align::FlexEnd,
            "stretch" => Align::Stretch,
            _ => Align::Auto,
        };
        self.align_self = align_type
    }

    pub fn set_align_items(&mut self, align: &str) {
        let align_type = match align {
            "center" => Align::Center,
            "flex-start" => Align::FlexStart,
            "flex-end" => Align::FlexEnd,
            "stretch" => Align::Stretch,
            _ => Align::Auto,
        };
        self.align_items = align_type
    }
}
