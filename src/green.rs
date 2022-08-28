use std::{ fmt, sync::Arc };

#[derive(Clone)]
pub struct GreenTree {
    data: Arc<GreenTreeData>,
}

#[derive(Clone)]
pub struct GreenTreeData {
    tag: String,
    width: isize,
    height: isize,
    direction: Direction,
    wrap: Wrap,
    grow: isize,
    shrink: isize,
    align_items: Align,
    justify_content: Align,
    order: usize,
    // todo more flex params
    children: Vec<GreenTree>,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Direction {
    Row,
    Column,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Align {
    Auto,
    Center,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Wrap {
    Wrap,
    NoWrap,
}

impl GreenTree {
    pub fn new(tag: impl Into<String>, width: usize, height: usize) -> GreenTreeData {
        GreenTreeData {
            tag: tag.into(),
            width: width as isize,
            height: height as isize,
            direction: Direction::Row,
            wrap: Wrap::NoWrap,
            align_items: Align::Auto,
            justify_content: Align::Auto,
            grow: 0,
            shrink: 1,
            order: 0,
            children: Vec::new(),
        }
    }

    pub fn tag(&self) -> &str {
        self.data.tag.as_str()
    }

    pub fn width(&self) -> isize {
        self.data.width
    }

    pub fn height(&self) -> isize {
        self.data.height
    }

    pub fn direction(&self) -> Direction {
        self.data.direction
    }

    pub fn wrap(&self) -> Wrap {
        self.data.wrap
    }

    pub fn grow(&self) -> isize {
        self.data.grow
    }

    pub fn shrink(&self) -> isize {
        self.data.shrink
    }

    pub fn order(&self) -> usize {
        self.data.order
    }

    pub fn align_items(&self) -> Align {
        self.data.align_items
    }

    pub fn justify_content(&self) -> Align {
        self.data.justify_content
    }

    pub fn children(&self) -> impl Iterator<Item = &GreenTree> {
        self.data.children.iter()
    }

    pub fn get_child(&self, index: usize) -> Option<&GreenTree> {
        self.data.children.get(index)
    }

    pub fn remove_child(&self, index: usize) -> GreenTree {
        let mut data = self.data.clone();
        Arc::make_mut(&mut data).children.remove(index);
        GreenTree { data }
    }

    pub fn insert_child(&self, index: usize, child: GreenTree) -> GreenTree {
        let mut data = self.data.clone();
        Arc::make_mut(&mut data).children.insert(index, child);
        GreenTree { data }
    }

    pub fn replace_child(&self, index: usize, child: GreenTree) -> GreenTree {
        let mut data = self.data.clone();
        Arc::make_mut(&mut data).children[index] = child;
        GreenTree { data }
    }
}

impl GreenTreeData {
    pub fn push(mut self, child: impl Into<GreenTree>) -> GreenTreeData {
        self.children.push(child.into());
        self
    }

    pub fn set(mut self, name: &str, value: &str) -> GreenTreeData {
        match name {
            "direction" => {
                self.direction = value.into();
            }
            "wrap" => {
                self.wrap = value.into();
            }
            "grow" => {
                self.grow = value.parse::<isize>().unwrap();
            }
            "shrink" => {
                self.shrink = value.parse::<isize>().unwrap();
            }
            "align-items" =>{
                self.align_items = value.into();
            }
            "justify-content" =>{
                self.justify_content = value.into();
            }
            _ => {}
        }
        self
    }
}

impl From<&str> for Direction {
    fn from(src: &str) -> Direction {
        if src.eq_ignore_ascii_case("row") { Direction::Row } else { Direction::Column }
    }
}

impl From<&str> for Wrap {
    fn from(src: &str) -> Wrap {
        if src.eq_ignore_ascii_case("wrap") { Wrap::Wrap } else { Wrap::NoWrap }
    }
}

impl From<&str> for Align {
    fn from(src: &str) -> Align {
        if src.eq_ignore_ascii_case("center") { Align::Center } else { Align::Auto }
    }
}

impl From<GreenTreeData> for GreenTree {
    fn from(data: GreenTreeData) -> GreenTree {
        GreenTree { data: Arc::new(data) }
    }
}

impl<T: Into<String>> From<T> for GreenTree {
    fn from(tag: T) -> Self {
        GreenTree::new(tag, 0, 0).into()
    }
}

impl fmt::Display for GreenTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_rec(f, 0, self)
    }
}

fn fmt_rec(f: &mut fmt::Formatter<'_>, lvl: usize, tree: &GreenTree) -> fmt::Result {
    writeln!(
        f,
        "{:indent$}{} {} {}",
        "",
        tree.tag(),
        tree.width(),
        tree.height(),
        indent = lvl * 2
    )?;
    for child in tree.children() {
        fmt_rec(f, lvl + 1, child)?;
    }
    Ok(())
}

impl fmt::Debug for GreenTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl PartialEq for GreenTree {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.data, &other.data)
    }
}

impl Eq for GreenTree {}