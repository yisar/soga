use soga::flexbox::FlexBox;
use soga::flexbox::FlexItem;
use std::convert::TryInto;

#[test]
fn grow() {
    let mut root = FlexItem::new(100, 240);

    root.direction = "column".try_into().unwrap();

    let mut child1 = FlexItem::new(60, 30);
    let mut child2 = FlexItem::new(60, 0);
    let mut child3 = FlexItem::new(60, 0);

    child1.grow = 0;
    child2.grow = 1;
    child3.grow = 2;

    root.add(child1);
    root.add(child2);
    root.add(child3);

    let mut flexbox = FlexBox::new();
    flexbox.layout(&mut root);

    assert_eq!(root.children[0].frame, [0, 0, 60, 30]);
    assert_eq!(root.children[1].frame, [0, 30, 60, 70]);
    assert_eq!(root.children[2].frame, [0, 100, 60, 140])
}
