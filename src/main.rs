pub mod flexbox;

fn main() {
    let flexbox = flexbox::FlexBox::new();
    let mut flexitem = flexbox::FlexBox::flex_item_with_size(&flexbox,100.0,100.0);
    let child = flexbox::FlexBox::flex_item_with_size(&flexbox,100.0,100.0);
    flexitem.flex_item_add(child);

    print!("{:#?}",flexitem);

}


