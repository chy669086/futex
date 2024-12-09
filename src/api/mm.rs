use lazy_static::lazy_static;

use super::TRANSLATE_VADDR;

lazy_static! {
    pub static ref translate_vaddr: fn(usize) -> Option<usize> = translate_vaddr_getter();
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
