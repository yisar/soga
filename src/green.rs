use std::{ fmt, sync::Arc };

#[derive(Clone)]
pub struct GreenTree {
    data: Arc<GreenTreeData>,
}

#[derive(Clone)]
pub struct GreenTreeData {
    tag: String,
    width: usize,
    height: usize,
    direction: Direction,
    wrap: Wrap,
    grow: usize,
    shrink: usize,
    grows: usize,
    shrinks: usize,
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
pub enum Wrap {
    Wrap,
    NoWrap,
}

impl GreenTree {
    pub fn new(tag: impl Into<String>, width: usize, height: usize) -> GreenTreeData {
        GreenTreeData {
            tag: tag.into(),
            width,
            height,
            direction: Direction::Row,
            wrap: Wrap::NoWrap,
            grow: 0,
            shrink: 1,
            grows: 0,
            shrinks: 1,
            order: 0,
            children: Vec::new(),
        }
    }

    pub fn tag(&self) -> &str {
        self.data.tag.as_str()
    }

    pub fn width(&self) -> usize {
        self.data.width
    }

    pub fn height(&self) -> usize {
        self.data.height
    }

    pub fn direction(&self) -> Direction {
        self.data.direction
    }

    pub fn wrap(&self) -> Wrap {
        self.data.wrap
    }

    pub fn grow(&self) -> usize {
        self.data.grow
    }

    pub fn shrink(&self) -> usize {
        self.data.shrink
    }

    pub fn order(&self) -> usize {
        self.data.order
    }

    pub fn grows(&self) -> usize {
        self.data.grows
    }

    pub fn shrinks(&self) -> usize {
        self.data.shrinks
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