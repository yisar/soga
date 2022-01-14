use soga::flexbox::FlexBox;
use soga::flexbox::FlexItem;
use std::convert::TryInto;

#[test]
fn order() {
    let mut root = FlexItem::new(100, 100);

    root.direction = "column".try_into().unwrap();

    let mut child1 = FlexItem::new(100, 100);
    let mut child2 = FlexItem::new(100, 100);
    let mut grandson = FlexItem::new(50,50);

    child1.add(grandson);

    root.add(child1);
    root.add(child2);

    let mut flexbox = FlexBox::new();
    flexbox.layout(&mut root);

    assert_eq!(root.children[0].children[0].frame, [0,0,50,50])
}
