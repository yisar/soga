use std::{ cell::Cell, cmp::Ordering, rc::{ self, Rc } };

pub(crate) trait Elem: std::fmt::Debug {
    fn prev(&self) -> &Cell<rc::Weak<Self>>;
    fn next(&self) -> &Cell<rc::Weak<Self>>;
    fn key(&self) -> &Cell<usize>;
}

pub(crate) fn unlink<E: Elem>(head: &Cell<rc::Weak<E>>, elem: &Rc<E>) {
    let prev = elem.prev().replace(Rc::downgrade(elem));
    let next = elem.next().replace(Rc::downgrade(elem));

    let single = prev.as_ptr() == Rc::as_ptr(elem);

    let old_head = head.take();
    head.set({
        let i_am_head = old_head.as_ptr() == (Rc::as_ptr(elem) as *const _);
        match (i_am_head, single) {
            (true, true) => rc::Weak::new(),
            (true, false) => next.clone(),
            (false, _) => old_head,
        }
    });

    if !single {
        let rc_prev = prev.upgrade().unwrap();
        let rc_next = next.upgrade().unwrap();
        rc_prev.next().set(next);
        rc_next.prev().set(prev);
    }
}

pub(crate) fn link<E: Elem>(head: &Cell<rc::Weak<E>>, elem: &mut Rc<E>) {
    let old_head = head.take();
    head.set(match old_head.upgrade() {
        Some(head) => {
            let mut curr = head.clone();
            let mut looped = false;
            loop {
                match elem.key().cmp(&curr.key()) {
                    Ordering::Equal => {
                        *elem = curr;
                        break;
                    }
                    Ordering::Greater if !looped => {
                        curr = {
                            let weak = curr.next().take();
                            let rc = weak.upgrade().unwrap();
                            curr.next().set(weak);
                            rc
                        };
                        looped |= Rc::ptr_eq(&curr, &head);
                    }
                    Ordering::Less | Ordering::Greater => {
                        let prev = curr.prev().replace(Rc::downgrade(&*elem));
                        prev.upgrade().unwrap().next().set(Rc::downgrade(&*elem));
                        elem.next().set(Rc::downgrade(&curr));
                        elem.prev().set(prev);
                        break;
                    }
                }
            }
            Rc::downgrade(if elem.key() < head.key() { &*elem } else { &head })
        }
        None => Rc::downgrade(&*elem),
    })
}

pub(crate) fn adjust<E: Elem>(elem: &Rc<E>, from: usize, by: isize) {
    let mut curr = elem.clone();
    loop {
        let key = curr.key().get();
        if key >= from {
            curr.key().set(((key as isize) + by) as usize);
        }
        curr = {
            let weak = curr.next().take();
            let rc = weak.upgrade().unwrap();
            curr.next().set(weak);
            rc
        };
        if Rc::ptr_eq(&curr, elem) {
            break;
        }
    }
}