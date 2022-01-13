use soga::flexbox::FlexBox;
use soga::flexbox::FlexItem;
use std::convert::TryInto;

#[test]
fn basis() {
    let mut root = FlexItem::new(100, 100);

    root.direction = "column".try_into().unwrap();

    let mut child1 = FlexItem::new(100, 40);
    
    child1.basis = 60;
    
    let mut child2 = FlexItem::new(100, 40);

    root.add(child1);
    root.add(child2);

    let mut flexbox = FlexBox::new();
    flexbox.layout(&mut root);

    assert_eq!(root.children[0].frame, [0, 0, 100, 60]);
    assert_eq!(root.children[1].frame, [0, 60, 100, 40]);
}



