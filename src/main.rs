pub mod green;
pub mod red;

use crate::green::*;
use crate::red::*;

fn make_tree() -> GreenTree {
    let tree: GreenTree = GreenTree::new("div", 10, 10)
        .push(GreenTree::new("text", 2, 2))
        .push(
            GreenTree::new("ul", 6, 6)
                .push(GreenTree::new("li", 2, 2))
                .push(GreenTree::new("li", 4, 4))
        )
        .into();
    tree.into()
}

fn main() {
    let tree = make_tree();
    // println!("{:#?}", tree);
    let flexbox = RedTree::new(tree);
    let res = flexbox.layout(&flexbox.data.green, 0, 0, 0, 0);
    println!("{:#?}", res);
}