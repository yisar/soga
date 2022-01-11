pub struct FlexBox {
    reverse: bool,
    vertical: bool,
    size_dim: f64,
    align_dim: f64,
    frame_pos: i64,
    frame_pos2: i64,
    frame_size: i64,
    frame_size2: i64,
    flex_dim: f64,
    flex_grows: i64,
    flex_shrinks: i64,
    pos2: f64,
}

pub struct FlexItem {
    width: f64,
    height: f64,
}

impl FlexBox {
    pub fn flex_item_with_size(&self, width: f64, height: f64) -> FlexItem {
        FlexItem { width, height }
    }

    pub fn flex_item_new(&self) -> FlexItem {
        FlexItem {
            width: 0.0,
            height: 0.0,
        }
    }

    pub fn flex_item_set_width(&mut self, mut child: FlexItem, width: f64) {
        child.width = width;
    }

    pub fn flex_item_set_height(&mut self, mut child: FlexItem, height: f64) {
        child.height = height;
    }

    pub fn flex_layout(&mut self, mut root: &FlexItem) {
        self.layout_item(root, root.width, root.height)
    }

    fn layout_item(&mut self,mut item:&FlexItem, width:f64,height:f64){
        
    }
}