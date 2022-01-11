pub mod flexbox;
use flexbox::FlexBox;
use flexbox::FlexItem;

fn main() {
    let mut root = FlexItem::new(100.0, 240.0);
    let mut child1 = FlexItem::new(60.0, 30.0);
    let mut child2 = FlexItem::new(60.0, 0.0);
    let mut child3 = FlexItem::new(60.0, 0.0);


    root.add(child1);
    root.add(child2);
    root.add(child3)

    let mut layout = FlexBox::new();
    layout.layout(&mut root);

    print!("{:#?}", root);
}
