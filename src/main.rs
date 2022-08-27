pub mod green;
pub mod red;
pub mod sll;
pub mod flex;

use crate::green::*;
use crate::red::*;
use crate::flex::*;

fn make_tree() -> RedTree {
    let tree: GreenTree = GreenTree::new("div", 10, 10) // 0 0 10 10
        .push(
            GreenTree::new("ul", 6, 6) // 0 0 6 6
                .set("wrap", "wrap")
                .push(GreenTree::new("li", 2, 2)) // 0 0 2 2
                .push(GreenTree::new("li", 5, 5)) // 0 2 5 5
                .push(GreenTree::new("li", 5, 5)) // 0 7 5 5
        )
        .into();

    tree.into()
}

fn main() {
    let tree = make_tree();
    let mut flexbox = FlexBox::new();
    flexbox.layout(tree);
    println!("{:#?}", flexbox.records);
}