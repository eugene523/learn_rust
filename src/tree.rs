pub struct Node<'a> {
    parent: Option<&'a Node<'a>>,
    data: u32,
    pub left: Option<Box<Node<'a>>>,
    pub right: Option<Box<Node<'a>>>
}

impl<'a> Node<'a> {
    pub fn new(data: u32) -> Node<'a> {
        Node { 
            parent: None, 
            data,
            left: None,
            right: None,
        }
    }

    pub fn set_left(&'a mut self, child: Option<Box<Node<'a>>>) {
        self.left = child;
        if let Some(left_box) = self.left.as_mut() {
            left_box.parent = Some(&self);
        }
        match self.left.as_mut() {
            // Not correct.
            Some(node) => { node.parent = Some(&self); }
            None => (),
        }
    }
}

impl<'a> Drop for Node<'a> {
    fn drop(&mut self) {
        println!("dropped {}", self.data);
    }
}
