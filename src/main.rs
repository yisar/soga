pub mod flexbox;
use flexbox::FlexItem;
use flexbox::FlexBox;

fn main() {
    let mut root = FlexItem::flex_item_with_size(100.0,100.0);
    let mut child = FlexItem::flex_item_new();
    child.flex_item_set_width(40.0);
    child.flex_item_set_height(40.0);
    
    root.flex_item_add(child);

    let mut layout = FlexBox::new();
    layout.layout_item(&root, 100.0, 100.0);



    print!("{:#?}",root);

}


