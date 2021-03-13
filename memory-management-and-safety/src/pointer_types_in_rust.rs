#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


#[cfg(test)]
mod box_basics {
    fn box_ref<T>(b: T) -> Box<T> {
        let a = b;
        Box::new(a)
    }

    struct Foo;

    #[test]
    fn test() {
        let boxed_one = Box::new(Foo);
        let unboxed_one = *boxed_one;
        box_ref(unboxed_one);
    }
}

#[cfg(test)]
mod recursive_type {
    struct Node {
        data: u32,
        // next: Option<Node>,
        next: Option<Box<Node>>,
    }

    #[test]
    fn test() {
        let a = Node {
            data: 33,
            next: None,
        };
        println!("{} {}", a.data, a.next.is_none())
    }
}


#[cfg(test)]
mod linked_list {
    use std::rc::Rc;

    #[derive(Debug)]
    struct LinkedList<T> {
        head: Option<Rc<Node<T>>>
    }

    #[derive(Debug)]
    struct Node<T> {
        next: Option<Rc<Node<T>>>,
        data: T,
    }

    impl<T> LinkedList<T> {
        fn new() -> Self {
            LinkedList { head: None }
        }

        fn append(&self, data: T) -> Self {
            LinkedList {
                head: Some(Rc::new(Node {
                    data,
                    next: self.head.clone(),
                }))
            }
        }
    }

    #[test]
    fn test() {
        let list_of_nums = LinkedList::new().append(1).append(2);
        println!("nums: {:?}", list_of_nums);

        let list_of_strings = LinkedList::new().append("foo").append("bar");
        println!("strings: {:?}", list_of_strings);
    }
}

#[cfg(test)]
mod rc_weak {
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    #[derive(Debug)]
    struct LinkedList<T> {
        head: Option<Rc<LinkedListNode<T>>>
    }

    #[derive(Debug)]
    struct LinkedListNode<T> {
        next: Option<Rc<LinkedListNode<T>>>,
        prev: RefCell<Option<Weak<LinkedListNode<T>>>>,
        data: T,
    }

    impl<T> LinkedList<T> {
        fn new() -> Self {
            LinkedList { head: None }
        }

        fn append(&mut self, data: T) -> Self {
            let new_node = Rc::new(LinkedListNode {
                data,
                next: self.head.clone(),
                prev: RefCell::new(None),
            });

            match self.head.clone() {
                Some(node) => {
                    let mut prev = node.prev.borrow_mut();
                    *prev = Some(Rc::downgrade(&new_node));
                }
                None => {}
            }

            LinkedList {
                head: Some(new_node)
            }
        }
    }

    #[test]
    fn test() {
        let list_of_nums = LinkedList::new().append(1).append(2).append(3);
        println!("nums: {:?}", list_of_nums);
    }
}


