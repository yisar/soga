pub mod green;

use crate::green::*;

fn make_tree() -> GreenTree {
    let tree: GreenTree = GreenTree::new("div")
        .push("111")
        .push(
            GreenTree::new("ul")
                .push(GreenTree::new("li").push("222"))
                .push(GreenTree::new("li").push("333"))
        )
        .into();
    tree.into()
}

fn main() {
    let tree = make_tree();
    println!("{:#?}", tree);
}


