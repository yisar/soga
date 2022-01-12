use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl<'a> TryFrom<&'a str> for Direction {
    type Error = ();

    fn try_from(src: &'a str) -> Result<Self, Self::Error> {
        if src.eq_ignore_ascii_case("row") {
            Ok(Self::Row)
        } else if src.eq_ignore_ascii_case("row-reverse") {
            Ok(Self::RowReverse)
        } else if src.eq_ignore_ascii_case("column") {
            Ok(Self::Column)
        } else if src.eq_ignore_ascii_case("column-reverse") {
            Ok(Self::ColumnReverse)
        } else {
            Err(())
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Row => write!(f, "row"),
            Self::RowReverse => write!(f, "row-reverse"),
            Self::Column => write!(f, "column"),
            Self::ColumnReverse => write!(f, "column-reverse"),
        }
    }
}

impl Into<String> for Direction {
    fn into(self) -> String {
        self.to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Align {
    Auto,
    Center,
    FlexStart,
    FlexEnd,
    Stretch,
    Between,
    Around,
}

impl<'a> From<&'a str> for Align {
    fn from(src: &'a str) -> Self {
        if src.eq_ignore_ascii_case("auto") {
            Self::Auto
        } else if src.eq_ignore_ascii_case("center") {
            Self::Center
        } else if src.eq_ignore_ascii_case("flex-start") {
            Self::FlexStart
        } else if src.eq_ignore_ascii_case("flex-end") {
            Self::FlexEnd
        } else if src.eq_ignore_ascii_case("stretch") {
            Self::Stretch
        } else if src.eq_ignore_ascii_case("between") {
            Self::Between
        } else if src.eq_ignore_ascii_case("around") {
            Self::Around
        } else {
            Self::Auto
        }
    }
}

impl Display for Align {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Center => write!(f, "center"),
            Self::FlexStart => write!(f, "flex-start"),
            Self::FlexEnd => write!(f, "flex-end"),
            Self::Stretch => write!(f, "stretch"),
            Self::Between => write!(f, "between"),
            Self::Around => write!(f, "around"),
        }
    }
}

impl Into<String> for Align {
    fn into(self) -> String {
        self.to_string()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Wrap {
    Wrap,
    NoWrap,
    WrapReverse,
}

impl<'a> TryFrom<&'a str> for Wrap {
    type Error = ();

    fn try_from(src: &'a str) -> Result<Self, Self::Error> {
        if src.eq_ignore_ascii_case("wrap") {
            Ok(Self::Wrap)
        } else if src.eq_ignore_ascii_case("nowrap") {
            Ok(Self::NoWrap)
        } else if src.eq_ignore_ascii_case("wrap-reverse") {
            Ok(Self::WrapReverse)
        } else {
            Err(())
        }
    }
}

impl Display for Wrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wrap => write!(f, "wrap"),
            Self::NoWrap => write!(f, "nowrap"),
            Self::WrapReverse => write!(f, "wrap-reverse"),
        }
    }
}

impl Into<String> for Wrap {
    fn into(self) -> String {
        self.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct FlexBox {
    reverse: bool,
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
    children: Vec<FlexItem>,
    frame: [usize; 4],
    pub grow: usize,
    pub shrink: usize,
    pub align_self: Align,
    pub align_items: Align,
    pub basis: usize,
    pub justify_content: Align,
    pub wrap: Wrap,
}

impl FlexBox {
    pub fn new() -> FlexBox {
        FlexBox {
            reverse: false,
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
            let mut wrap_dim = flex_dim;
            match item.wrap {
                Wrap::NoWrap => {
                    if flex_dim > 0 {
                        if child.grow != 0 {
                            size = (flex_dim / self.grows) * child.grow;
                        }
                    } else {
                        if child.shrink != 0 {
                            size = (flex_dim / self.shrinks) * child.shrink;
                        }
                    }
                }
                Wrap::Wrap => {
                    let child_size = child.frame[self.size_i];
                    if wrap_dim >= child_size {
                        wrap_dim -= child_size;
                    } else {
                        wrap_dim = flex_dim;
                        pos += child.frame[self.size_i];
                    }
                }
                Wrap::WrapReverse => {
                    unimplemented!()
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
        self.children.push(child)
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
            frame: [0; 4],
            grow: 0,
            shrink: 0,
            basis: 0,
            align_self: Align::Auto,
            align_items: Align::Auto,
            justify_content: Align::Auto,
            wrap: Wrap::NoWrap,
        }
    }

    pub fn default() -> FlexItem {
        FlexItem {
            width: 0,
            height: 0,
            direction: Direction::Row,
            children: vec![],
            frame: [0; 4],
            grow: 0,
            shrink: 0,
            basis: 0,
            align_self: Align::Auto,
            align_items: Align::Auto,
            justify_content: Align::Auto,
            wrap: Wrap::NoWrap,
        }
    }
}
