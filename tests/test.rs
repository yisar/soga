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

#[test]
fn justify_content() {
    let mut root = FlexItem::new(100, 300);

    root.direction = "column".try_into().unwrap();
    root.justify_content = "center".try_into().unwrap();

    let mut child1 = FlexItem::new(50, 100);
    let mut child2 = FlexItem::new(50, 100);

    root.add(child1);
    root.add(child2);

    let mut flexbox = FlexBox::new();
    flexbox.layout(&mut root);

    assert_eq!(root.children[0].frame, [0, 50, 50, 100]);
    assert_eq!(root.children[1].frame, [0, 150, 50, 100]);
}

#[test]
fn align_items() {
    let mut root = FlexItem::new(100, 10);

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
