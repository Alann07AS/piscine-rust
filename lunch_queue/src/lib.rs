#[derive(Debug)]

pub struct Queue {
    pub node: Link,
}

pub type Link = Box<Option<Person>>;

#[derive(Debug)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next_person: Link,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { node: Box::new(None) }
    }
    pub fn add(&mut self, name: String, discount: i32) {
        self.node =  Box::new(Some(Person { discount, name, next_person: Box::new(self.node.take()) }));
    }
    pub fn invert_queue(&mut self) {
        let mut node: &Link= &self.node;
        let mut ref_list: Vec<&Link> = vec![];
        while check_next(node) {
            ref_list.push(&node);
            node = &node.as_ref().as_ref().unwrap().next_person;
        }
        let mut rev_ref_list = ref_list.clone();
        rev_ref_list.reverse();
        ref_list.into_iter().for_each(|n| {
            n.as_mut().unwrap().next_person = rev_ref_list.pop().unwrap().take();
        })
    }
    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut node: &mut Link= &mut self.node;
        // let mut link_before: &mut Link= &mut None;
        loop  {
            if check_next(&node) && !check_next(&node.as_ref().as_ref().unwrap().next_person) {
                let p = node.as_mut().as_mut().unwrap();
                let p_next = p.next_person.as_mut().as_mut().unwrap(); 
                let data = (p_next.name.clone(), p_next.discount);
                p.next_person = Box::new(None);
                return Some(data);
            } else {
                node = &mut node.as_mut().as_mut().unwrap().next_person;
            }

        }
    }
    pub fn search(&self, name: &str) -> Option<(String, i32)> {
        let mut node:  &Link= &self.node;
        // let mut link_before: &mut Link= &mut None;
        loop  {
            if node.is_some() {
                let p = &node.unwrap();
                if p.name == name {
                    return Some((name.to_owned(), p.discount));
                }
            }
            if check_next(&node)  {
                node = &node.as_ref().as_ref().unwrap().next_person;
            } else {
                return None;
            }

        }
    }
}

fn check_next(l :&Link) -> bool {
    match *l.as_ref().as_ref().unwrap().next_person {
        None    => false,
        Some(_) => true,
    }
}


// let mut link: &mut Link= &mut self.node;
// loop  {
//     match link {
//         None => {
//             *link = Some(Person {discount, name, next_person: Box::new(None)});
//             break;
//         },
//         Some(p) => {
//             link = &mut *p.next_person;
//         },
//     }
// }



// pub fn add(&mut self, name: String, discount: i32) {
//     let mut link = self.node.take();
//     let new_person = Some(Person {
//         discount,
//         name,
//         next_person: Box::new(link.take()),
//     });
//     self.node = Box::new(new_person);
// }