pub mod flexbox;

use crate::flexbox::FlexItem;
use crate::flexbox::FlexBox;
use std::convert::TryInto;


fn main() {
    let mut root = FlexItem::new(100, 100);

    root.direction = "column".try_into().unwrap();

    let mut child1 = FlexItem::new(100, 100);
    let mut child2 = FlexItem::new(100, 100);
    let mut grandson = FlexItem::new(50,50);

    root.add(child1);
    root.add(child2);

    child1.add(grandson);

    let mut flexbox = FlexBox::new();
    flexbox.layout(&mut root);

    println!("{:#?}", root);

}


