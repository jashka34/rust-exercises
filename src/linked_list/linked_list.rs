#[allow(dead_code)]
pub struct ListItem<T> {
    data: Box<T>,
    next: Option<Box<ListItem<T>>>,
}

#[allow(dead_code)]
pub struct LinkedLinst<T> {
    head: ListItem<T>,
}

#[allow(dead_code)]
impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data),
            next: None,
        }
    }
    fn data(&self) -> &T {
        self.data.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;
    // use linked_list::ListItem;
    #[test]
    fn test_list_item_new() {
        let lls = ListItem::new("test");
        assert_eq!(*lls.data(), "test");
        assert!(lls.next.is_none());
    }
}
