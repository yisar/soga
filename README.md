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
use soga::flexbox::FlexBox;
use soga::flexbox::FlexItem;
use std::convert::TryInto;

fn main() {
    let mut root = FlexItem::new(100, 100);

    let mut child1 = FlexItem::new(100, 50);
    let mut child2 = FlexItem::new(100, 50);

    root.add(child1);
    root.add(child2);

    let mut flexbox = FlexBox::new();
    flexbox.layout(&mut root);

    assert_eq!(root.children[0].frame, [0, 0, 100, 50]);
    assert_eq!(root.children[1].frame, [50, 0, 100, 50]);
}

```

### License

MIT ©yisar inspired by yoga.
