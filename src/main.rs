pub mod flexbox;

use crate::flexbox::FlexItem;
use crate::flexbox::FlexBox;
use std::convert::TryInto;


fn main() {
    let mut root = FlexItem::new(100, 100);

    root.direction = "column".try_into().unwrap();

    let mut child1 = FlexItem::new(100, 100);
    let mut child2 = FlexItem::new(100, 100);

    child1.shrink = 2;
    child2.shrink = 3;

    root.add(child1);
    root.add(child2);

    let mut flexbox = FlexBox::new();
    flexbox.layout(&mut root);

    // assert_eq!(root.children[0].frame, [0, 0, 100, 60]);
    // assert_eq!(root.children[1].frame, [0, 60, 100, 40]);

    // assert_eq!(root.children[0].frame, [25, 0, 50, 25]);
    // assert_eq!(root.children[1].frame, [25, 25, 50, 25]);
    // assert_eq!(root.children[2].frame, [25, 50, 50, 25])

    println!("{:#?}", root);

}


