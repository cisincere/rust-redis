use std::ptr::NonNull;

const AL_START_HEAD: i32 = 0;
const AL_START_TAIL: i32 = 0;

trait ListMemoryOperation<T> {
    fn list_create() -> BindList<T>;
    fn list_release(list: BindList<T>) -> BindList<T>;
    fn list_add_node_head(list: BindList<T>, value: T) -> BindList<T>;
}

impl<T> BindList<T> {
    pub const fn list_create() -> Self {
        BindList { head: None, tail: None, len: 0 }
    }
}

pub struct BindListNode<T> {
    prev: Option<NonNull<BindListNode<T>>>,
    next: Option<NonNull<BindListNode<T>>>,
    value: T
}
pub struct BindListIter<'a, T: 'a> {
    next: Option<NonNull<BindListNode<T>>>,
    direction: i8,
}
pub struct BindList<T> {
    head: Option<NonNull<BindListNode<T>>>,
    tail: Option<NonNull<BindListNode<T>>>,
    len: usize
}
impl<T> BindList<T> {

    fn list_len(&self) -> usize {
        return self.len
    }

    fn list_first(&self) -> Option<NonNull<BindListNode<T>>> {
        return self.head
    }

    fn list_last(&self) -> Option<NonNull<BindListNode<T>>> {
        return self.tail
    }

    fn list_prev_node(&self, node: BindListNode<T>) -> Option<NonNull<BindListNode<T>>> {
        return node.prev
    }

    fn list_next_node(&self, node: BindListNode<T>) -> Option<NonNull<BindListNode<T>>> {
        return node.next
    }

    fn list_node_value(&self, node: BindListNode<T>) -> Option<T> {
        return node.value
    }
}