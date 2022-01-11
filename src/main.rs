pub mod flexbox;
use flexbox::FlexItem;
use flexbox::FlexBox;

fn main() {
    let mut root = FlexItem::flex_item_with_size(100.0,100.0);
    let mut child1 = FlexItem::flex_item_new();
    child1.flex_item_set_width(40.0);
    child1.flex_item_set_height(40.0);

    let mut child2 = FlexItem::flex_item_with_size(20.0, 10.0);
    
    root.flex_item_add(child1);
    root.flex_item_add(child2);

    let mut layout = FlexBox::new();
    layout.layout_item(&mut root);



    print!("{:#?}",root);

}


