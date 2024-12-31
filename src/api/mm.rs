use lazy_static::lazy_static;

use super::{COPY_FROM_USER, COPY_TO_USER, TRANSLATE_VADDR};

lazy_static! {
    pub static ref translate_vaddr: fn(usize) -> Option<usize> = translate_vaddr_getter();
}

lazy_static! {
    pub static ref copy_from_user: fn(usize, *mut u8, usize) -> usize = copy_from_user_getter();
}

lazy_static! {
    pub static ref copy_to_user: fn(usize, *mut u8, usize) -> usize = copy_to_user_getter();
}

fn copy_to_user_getter() -> fn(usize, *mut u8, usize) -> usize {
    let mut iter = COPY_TO_USER.iter();
    let Some(handler) = iter.next() else {
        panic!("No handler for COPY_TO_USER");
    };

    assert!(iter.next().is_none(), "Multiple handlers for COPY_TO_USER");

    drop(iter);

    handler.clone()
}

fn copy_from_user_getter() -> fn(usize, *mut u8, usize) -> usize {
    let mut iter = COPY_FROM_USER.iter();
    let Some(handler) = iter.next() else {
        panic!("No handler for COPY_FROM_USER");
    };

    assert!(
        iter.next().is_none(),
        "Multiple handlers for COPY_FROM_USER"
    );

    drop(iter);

    handler.clone()
}

fn translate_vaddr_getter() -> fn(usize) -> Option<usize> {
    let mut iter = TRANSLATE_VADDR.iter();
    let Some(handler) = iter.next() else {
        panic!("No handler for TRANSLATE_VADDR");
    };

    assert!(
        iter.next().is_none(),
        "Multiple handlers for TRANSLATE_VADDR"
    );

    drop(iter);

    handler.clone()
}
