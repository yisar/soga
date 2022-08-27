use std::{ cell::{ Cell, RefCell }, fmt, iter, rc::{ Rc, Weak } };
use crate::sll;
use crate::green;

pub use green::{ GreenTree, GreenTreeData };

#[derive(Clone)]
pub struct RedTree {
    data: Rc<RedTreeData>,
}

struct RedTreeData {
    green: RefCell<GreenTree>,
    parent: Cell<Option<RedTree>>,
    index: Cell<usize>,
    first: Cell<Weak<RedTreeData>>,
    next: Cell<Weak<RedTreeData>>,
    prev: Cell<Weak<RedTreeData>>,
}

impl sll::Elem for RedTreeData {
    fn prev(&self) -> &Cell<Weak<Self>> {
        &self.prev
    }
    fn next(&self) -> &Cell<Weak<Self>> {
        &self.next
    }
    fn key(&self) -> &Cell<usize> {
        &self.index
    }
}

impl RedTree {
    fn new(green: GreenTree, parent: Option<RedTree>, index: usize) -> RedTree {
        let data = RedTreeData {
            green: RefCell::new(green),
            parent: Cell::new(parent),
            index: Cell::new(index),
            first: Default::default(),
            next: Default::default(),
            prev: Default::default(),
        };
        let data = Rc::new(data);
        data.next.set(Rc::downgrade(&data));
        data.prev.set(Rc::downgrade(&data));
        RedTree { data }
    }
    pub fn tag(&self) -> String {
        self.data.green.borrow().tag().to_string()
    }

    pub fn width(&self) -> usize {
        self.data.green.borrow().width()
    }

    pub fn height(&self) -> usize {
        self.data.green.borrow().height()
    }

    pub fn wrap(&self) -> green::Wrap {
        self.data.green.borrow().wrap()
    }

    pub fn direction(&self) -> green::Direction {
        self.data.green.borrow().direction()
    }

    pub fn grow(&self) -> usize {
        self.data.green.borrow().grow()
    }

    pub fn shrink(&self) -> usize {
        self.data.green.borrow().shrink()
    }

    pub fn order(&self) -> usize {
        self.data.green.borrow().order()
    }

    pub fn grows(&self) -> usize {
        self.data.green.borrow().grows()
    }

    pub fn shrinks(&self) -> usize {
        self.data.green.borrow().shrinks()
    }

    pub fn parent(&self) -> Option<RedTree> {
        let ret = self.data.parent.take();
        self.data.parent.set(ret.clone());
        ret
    }
    pub fn first_child(&self) -> Option<RedTree> {
        self.get_child(0)
    }
    pub fn next_sibling(&self) -> Option<RedTree> {
        let parent = self.parent()?;
        let index = self.data.index.get() + 1;
        parent.get_child(index)
    }
    pub fn prev_sibling(&self) -> Option<RedTree> {
        let parent = self.parent()?;
        let index = self.data.index.get().checked_sub(1)?;
        parent.get_child(index)
    }
    fn get_child(&self, index: usize) -> Option<RedTree> {
        let green = self.data.green.borrow().get_child(index).cloned()?;
        let parent = Some(self.clone());
        let mut res = RedTree::new(green, parent, index);
        sll::link(&self.data.first, &mut res.data);
        Some(res)
    }

    pub fn children(&self) -> impl Iterator<Item = RedTree> {
        iter::successors(self.first_child(), |it| it.next_sibling())
    }
    pub fn find(&self, tag: &str) -> Option<RedTree> {
        self.children().find(|it| it.tag() == tag)
    }

    pub fn insert_child(&self, index: usize, mut child: RedTree) {
        assert!(child.parent().is_none());
        let weak = self.data.first.take();
        let first = weak.upgrade();
        self.data.first.set(weak);
        if let Some(first) = first {
            sll::adjust(&first, index, 1);
        }
        sll::link(&self.data.first, &mut child.data);

        let green = self.data.green.borrow().insert_child(index, child.data.green.borrow().clone());
        self.replace_green(green)
    }
    pub fn detach(&self) {
        if let Some(parent) = self.parent() {
            let green = parent.data.green.borrow().remove_child(self.data.index.get());
            parent.replace_green(green);
        }
        sll::adjust(&self.data, self.data.index.get() + 1, -1);
        self.unlink();
    }
    fn replace_green(&self, mut green: GreenTree) {
        let mut node = self.clone();
        loop {
            *node.data.green.borrow_mut() = green.clone();
            match node.parent() {
                Some(parent) => {
                    green = parent.data.green.borrow().replace_child(node.data.index.get(), green);
                    node = parent;
                }
                None => {
                    return;
                }
            }
        }
    }
    fn unlink(&self) {
        let dummy;
        let parent = self.data.parent.take();
        let head = match parent.as_ref() {
            Some(it) => &it.data.first,
            None => {
                dummy = Cell::new(Weak::new());
                &dummy
            }
        };
        sll::unlink(head, &self.data);
        self.data.index.set(0);
    }
}

impl Drop for RedTree {
    fn drop(&mut self) {
        if Rc::strong_count(&self.data) == 1 {
            assert!(self.data.first.take().strong_count() == 0);
            self.unlink()
        }
    }
}

impl From<GreenTree> for RedTree {
    fn from(green: GreenTree) -> Self {
        RedTree::new(green, None, !0)
    }
}

impl fmt::Display for RedTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.data.green.borrow(), f)
    }
}

impl fmt::Debug for RedTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl PartialEq for RedTree {
    fn eq(&self, other: &RedTree) -> bool {
        self.data.green == other.data.green
    }
}

impl Eq for RedTree {}

impl fmt::Debug for RedTreeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.green.borrow(), f)
    }
}