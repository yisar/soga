use soga::flexbox::FlexBox;
use soga::flexbox::FlexItem;
use std::convert::TryInto;

#[test]
fn justify_content() {
    let mut root = FlexItem::new(100, 300);

    root.direction = "column".try_into().unwrap();
    root.justify_content = "center".try_into().unwrap();

    let child1 = FlexItem::new(50, 100);
    let child2 = FlexItem::new(50, 100);

    root.add(child1);
    root.add(child2);

    let mut flexbox = FlexBox::new();
    flexbox.layout(&mut root);

    assert_eq!(root.children[0].frame, [0, 50, 50, 100]);
    assert_eq!(root.children[1].frame, [0, 150, 50, 100]);
}
