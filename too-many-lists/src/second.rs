#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    // change List type to IntoIter
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // change List type to Iter
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    // change List type to IterMut
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_deref_mut() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
        }

        println!("list has been cleared to empty");
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map::<&Node<T>, _>(|node| { &node });
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use crate::second::List;

    #[test]
    fn test_all() {
        let mut list = List::new();
        println!("list: {:?}", list);
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        println!("list: {:?}", list);

        let peek_val = list.peek();
        println!("peek_val: {:?}", peek_val);
        assert_eq!(peek_val, Some(&3));

        let peek_mut_val = list.peek_mut();
        println!("peek_mut_val: {:?}", peek_mut_val);
        assert_eq!(peek_mut_val, Some(&mut 3));

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        println!("list: {:?}", list);
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        while let Some(cur) = iter.next() {
            print!("[{:?}] => ", cur)
        }
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        while let Some(cur) = iter.next() {
            print!("[{:?}] => ", cur)
        }
        println!();

        println!("after: {:?}", list);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        while let Some(cur) = iter.next() {
            print!("[{:?}] => ", cur);
            *cur = *cur + 1;
        }
        println!();

        println!("after: {:?}", list);
    }
}