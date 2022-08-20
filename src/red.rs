use crate::green::{ GreenTree, GreenTreeData };
use std::{ fmt, sync::Arc };

#[derive(Clone)]
pub struct RedTree {
    pub data: RedTreeData,
}

#[derive(Clone)]
pub struct RedTreeData {
    pub frame: [usize; 4],
    pub green: GreenTree,
    pub children: Vec<RedTreeData>,
}

impl RedTree {
    pub fn new(green: GreenTree) -> RedTree {
        RedTree {
            data: RedTreeData {
                frame: [0; 4],
                green: green,
                children: Vec::new(),
            },
        }
    }
    pub fn layout(
        &self,
        green: &GreenTree,
        sx: usize,
        sy: usize,
        px: usize,
        py: usize
    ) -> RedTreeData {
        let mut x = 0;
        let mut y = 0;

        let children = green
            .children()
            .map(|child| {
                let ret = self.layout(child, x, y, sx, sy);
                x += child.width();
                y += child.height();
                ret
            })
            .collect::<Vec<_>>();

        RedTreeData {
            frame: [px + sx, py + sy, green.width(), green.height()],
            children,
            green: green.clone(),
        }
    }
}

impl fmt::Display for RedTreeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_rec(f, 0, self)
    }
}

impl fmt::Debug for RedTreeData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

fn fmt_rec(f: &mut fmt::Formatter<'_>, lvl: usize, tree: &RedTreeData) -> fmt::Result {
    writeln!(
        f,
        "{:indent$}{} [{}, {}, {}, {}]",
        "",
        tree.green.tag(),
        tree.frame[0],
        tree.frame[1],
        tree.frame[2],
        tree.frame[3],
        indent = lvl * 2
    )?;
    for child in tree.children.iter() {
        fmt_rec(f, lvl + 1, child)?;
    }
    Ok(())
}