use soga::flexbox::FlexBox;
use soga::flexbox::FlexItem;
use std::convert::TryInto;

#[test]
fn order() {
    let mut root = FlexItem::new(200, 200);

    root.direction = "column".try_into().unwrap();

    let mut child1 = FlexItem::new(50, 50);
    let mut child2 = FlexItem::new(50, 50);
    let mut child3 = FlexItem::new(50, 50);

    child1.order = 1;
    child2.order = 3;
    child3.order = 2;

    root.add(child1);
    root.add(child2);
    root.add(child3);

    assert_eq!(root.children[2].order, 2);

    let mut flexbox = FlexBox::new();
    flexbox.layout(&mut root);

    assert_eq!(root.children[2].frame, [0, 100, 50, 50]);
    assert_eq!(root.children[2].order, 3)
}
