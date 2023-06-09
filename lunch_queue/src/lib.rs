#[derive(Debug)]

pub struct Queue {
    pub node: Link,
}

pub type Link = Option<Person>;

#[derive(Debug)]
pub struct Person {
    pub discount: i32,
    pub name: String,
    pub next_person: Box<Link>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue { node: None }
    }
    pub fn add(&mut self, name: String, discount: i32) {
        self.node =  Some(Person { discount, name, next_person: Box::new(self.node.take()) });
    }
    pub fn invert_queue(&mut self) {
        
    }
    pub fn rm(&mut self) -> Option<(String, i32)> {
        let mut node: &mut Link= &mut self.node;
        // let mut link_before: &mut Link= &mut None;
        loop  {
            if check_next(&node) && !check_next(&node.as_ref().unwrap().next_person) {
                let p = &mut node.as_mut().unwrap();
                let p_next = &mut p.next_person.as_mut().as_mut().unwrap(); 
                let data = (p_next.name.clone(), p_next.discount);
                p.next_person = Box::new(None);
                return Some(data);
            } else {
                node = &mut *node.as_mut().unwrap().next_person;
            }

        }
    }
    // pub fn search(&self, name: &str) -> Option<(String, i32)> {

    // }
}

fn check_next(l :&Link) -> bool {
    match *l.as_ref().unwrap().next_person {
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