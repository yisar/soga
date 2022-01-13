use soga::flexbox::FlexBox;
use soga::flexbox::FlexItem;
use std::convert::TryInto;

#[test]
fn align_items() {
    let mut root = FlexItem::new(100, 100);

    root.direction = "column".try_into().unwrap();
    root.align_items = "center".try_into().unwrap();

    let mut child1 = FlexItem::new(50, 25);
    let mut child2 = FlexItem::new(50, 25);
    let mut child3 = FlexItem::new(50, 25);

    root.add(child1);
    root.add(child2);
    root.add(child3);

    let mut flexbox = FlexBox::new();
    flexbox.layout(&mut root);

    assert_eq!(root.children[0].frame, [25, 0, 50, 25]);
    assert_eq!(root.children[1].frame, [25, 25, 50, 25]);
    assert_eq!(root.children[2].frame, [25, 50, 50, 25])
}
