pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }
    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        self.ref_list = self.ref_list.drain(..).filter(|r|{
            element.cmp(&r) != std::cmp::Ordering::Equal
        }).collect()
    }
    // pub fn rm_all_ref(&mut self, element: Rc<String>) {
    //     let mut new_list = Vec::with_capacity(self.ref_list.len());
    //     for r in self.ref_list.iter() {
    //         if element.cmp(&r) != std::cmp::Ordering::Equal {
    //             new_list.push(r.clone());
    //         }
    //     }
    //     self.ref_list = new_list;
    // }
    
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}