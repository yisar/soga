use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use core::cell::RefCell;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlexError {
    #[error("bad direction value `{0}`")]
    BadDirection(String),
    #[error("bad wrap value `{0}`")]
    BadWrap(String),
}

pub use FlexError as Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl<'a> TryFrom<&'a str> for Direction {
    type Error = FlexError;

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
            Err(FlexError::BadDirection(src.to_string()))
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
    type Error = FlexError;

    fn try_from(src: &'a str) -> Result<Self, Self::Error> {
        if src.eq_ignore_ascii_case("wrap") {
            Ok(Self::Wrap)
        } else if src.eq_ignore_ascii_case("nowrap") {
            Ok(Self::NoWrap)
        } else if src.eq_ignore_ascii_case("wrap-reverse") {
            Ok(Self::WrapReverse)
        } else {
            Err(FlexError::BadWrap(src.to_string()))
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
    size1: usize,
    pos1: usize,
    size2: usize,
    pos2: usize,
    grows: isize,
    shrinks: isize,
}

#[derive(Clone, Debug)]
pub struct FlexItem {
    pub width: isize,
    pub height: isize,
    pub direction: Direction,
    pub children: Vec<FlexItem>,
    pub frame: [isize; 4],
    pub grow: isize,
    pub shrink: isize,
    pub align_self: Align,
    pub align_items: Align,
    pub basis: isize,
    pub justify_content: Align,
    pub wrap: Wrap,
    pub order: isize,
}

impl FlexBox {
    pub fn new() -> FlexBox {
        FlexBox {
            reverse: false,
            grows: 0,
            shrinks: 0,
            pos1: 0,
            size1: 0,
            size2: 0,
            pos2: 0,
        }
    }

    pub fn layout(&mut self, item: &mut FlexItem) {
        let mut flex_dim: isize = 0;
        let mut align_dim = 0;
        let mut size = 0;

        match &item.direction {
            Direction::Row => {
                flex_dim = item.width as isize;
                align_dim = item.height as isize;
                self.pos1 = 0;
                self.pos2 = 1;
                self.size1 = 2;
                self.size2 = 3;
            }
            Direction::RowReverse => {
                self.reverse = true;
            }
            Direction::Column => {
                flex_dim = item.height as isize;
                align_dim = item.width as isize;
                self.pos1 = 1;
                self.pos2 = 0;
                self.size1 = 3;
                self.size2 = 2;
            }
            Direction::ColumnReverse => self.reverse = false,
        }

        item.children
            .sort_by(|a, b| a.order.partial_cmp(&b.order).unwrap());

        let mut pos = if self.reverse { flex_dim } else { 0 };

        for child in item.children.iter_mut() {

            self.layout(child);

            child.frame[0] = 0;
            child.frame[1] = 0;
            child.frame[2] = child.width;
            child.frame[3] = child.height;

            flex_dim -= child.frame[self.size1] as isize;

            self.grows += child.grow;
            self.shrinks += child.shrink;

            if child.basis > 0 {
                child.frame[self.size1] = child.basis;
            }
        }

        for child in item.children.iter_mut() {
            match item.wrap {
                Wrap::NoWrap => {
                    if flex_dim > 0 {
                        if child.grow != 0 {
                            size = (flex_dim / self.grows) * child.grow;
                        }
                    } else if flex_dim < 0 {
                        if child.shrink != 0 {
                            size = (flex_dim / self.shrinks) * child.shrink;
                        }
                    }
                }
                Wrap::Wrap => {}
                Wrap::WrapReverse => {
                    unimplemented!()
                }
            }

            child.frame[self.size1] += size;

            let mut spacing = 0;

            if pos == 0 {
                match item.justify_content {
                    Align::FlexEnd => {
                        pos = flex_dim;
                    }
                    Align::Center => {
                        pos = flex_dim / 2;
                    }
                    Align::Between => {
                        if child.children.len() > 0 {
                            spacing = flex_dim / (child.children.len() - 1) as isize;
                        }
                    }
                    Align::Around => {
                        if child.children.len() > 0 {
                            spacing = flex_dim / child.children.len() as isize;
                            pos = spacing / 2;
                        }
                    }
                    _ => {}
                }
            }

            child.frame[self.pos1] += pos + spacing;

            if self.reverse {
                child.frame[self.pos1] = pos;
                pos -= child.frame[self.size1];
            } else {
                child.frame[self.pos1] = pos;
                pos += child.frame[self.size1]
            }

            let mut align = 0;
            let mut align_type = &item.align_self;

            if align_type == &Align::Auto {
                align_type = &item.align_items;
            }

            match align_type {
                Align::Auto => {}
                Align::FlexStart => {}
                Align::Center => {
                    align = (align_dim / 2) - (child.frame[self.size2] / 2);
                }
                Align::FlexEnd => {
                    align = align_dim - child.frame[self.size2];
                }
                Align::Stretch => {
                    align = 0;
                    child.frame[self.size2] = align_dim;
                }
                _ => {}
            }
            child.frame[self.pos2] = align;
        }
    }

    pub fn flex_layout(&mut self, root: &mut FlexItem) {
        self.layout(root)
    }
}

impl FlexItem {
    pub fn add(&mut self, child: FlexItem) {
        self.children.push(child);
    }

    pub fn delete(&mut self, index: usize) {
        self.children.remove(index);
    }

    pub fn insert(&mut self, index: usize, child: FlexItem) {
        self.children.insert(index, child)
    }

    pub fn new(width: isize, height: isize) -> FlexItem {
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
            order: 0,
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
            order: 0,
        }
    }
}
