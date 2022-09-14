pub mod green;
pub mod red;
pub mod sll;
pub mod flex;
pub mod idom;

use crate::green::*;
use crate::red::*;
use crate::flex::*;
use crate::idom::*;

// fn make_tree() -> RedTree {
//     let tree: GreenTree = GreenTree::new("div", 10, 10) // 0 0 10 10
//         .push(
//             GreenTree::new("ul", 6, 6) // 0 0 6 6
//                 .push(GreenTree::new("li", 0, 6).set("grow", "1")) // 0 0 1 6
//                 .push(GreenTree::new("li", 0, 6).set("grow", "5")) // 1 0 5 6
//         )
//         .into();

//     tree.into()
// }

// fn main() {
//     let tree = make_tree();
//     let mut flexbox = FlexBox::new();
//     flexbox.layout(tree);
//     println!("{:#?}", flexbox.records);
// }

fn main(){
    println!("hello world");
}