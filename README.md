# soga

Yoga alternative

[![Build Status](https://github.com/yisar/soga/actions/workflows/rust.yml/badge.svg)](https://github.com/yisar/soga/actions)
[![crates.io](https://img.shields.io/crates/v/soga.svg)](https://crates.io/crates/soga)
[![crates.io](https://docs.rs/soga/badge.svg)](https://docs.rs/soga/)
[![license](https://img.shields.io/github/license/yisar/soga.svg)](https://github.com/yisar/deku)

### Test

```shell
cargo test
```

### Use

```rs
use crate::soga::green::*;
use crate::soga::red::*;
use crate::soga::flex::*;

fn make_tree() -> RedTree {
    let tree: GreenTree = GreenTree::new("div", 10, 10) // 0 0 10 10
        .push(
            GreenTree::new("ul", 6, 6) // 0 0 6 6
                .push(GreenTree::new("li", 0, 6).set("grow", "1")) // 0 0 1 6
                .push(GreenTree::new("li", 0, 6).set("grow", "5")) // 1 0 5 6
        )
        .into();

    tree.into()
}

fn main() {
    let tree = make_tree();
    let mut flexbox = FlexBox::new();
    flexbox.layout(tree);
    assert_eq!(flexbox.records[0].rect, [0, 0, 10, 10]);
}
```

### License

MIT ©yisar inspired by yoga.
