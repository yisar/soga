pub mod flexbox;

use crate::flexbox::FlexItem;
use crate::flexbox::FlexBox;
use std::convert::TryInto;


fn main() {
    let mut root = FlexItem::new(100, 300);

    root.direction = "column".try_into().unwrap();
    root.justify_content = "center".try_into().unwrap();

    let mut child1 = FlexItem::new(50, 100);
    let mut child2 = FlexItem::new(50, 100);

    root.add(child1);
    root.add(child2);

    let mut flexbox = FlexBox::new();
    flexbox.layout(&mut root);

    print!("{:#?}", root);

}


