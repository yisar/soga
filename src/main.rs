pub mod green;

use crate::green::*;

fn make_tree() -> GreenTree {
    let tree: GreenTree = GreenTree::new("div", 10, 10)
        .push("111")
        .push(
            GreenTree::new("ul", 6, 6)
                .push(GreenTree::new("li", 2, 2).push("222"))
                .push(GreenTree::new("li", 4, 4).push("333"))
        )
        .into();
    tree.into()
}

fn main() {
    let tree = make_tree();
    println!("{:#?}", tree);
}