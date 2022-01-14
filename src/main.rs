pub mod flexbox;

use crate::flexbox::FlexItem;
use crate::flexbox::FlexBox;
use std::convert::TryInto;


fn main() {
    let mut root = FlexItem::new(100, 100);

    root.direction = "column".try_into().unwrap();
    root.align_items = "center".try_into().unwrap();

    let child1 = FlexItem::new(50, 25);
    let child2 = FlexItem::new(50, 25);
    let child3 = FlexItem::new(50, 25);

    root.add(child1);
    root.add(child2);
    root.add(child3);

    let mut flexbox = FlexBox::new();
    flexbox.layout(&mut root);


    println!("{:#?}", root);

}


