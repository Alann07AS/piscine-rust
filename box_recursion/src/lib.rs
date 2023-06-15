#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: String,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> WorkEnvironment {
        WorkEnvironment { grade: None }
    }
    pub fn add_worker(&mut self, role: String, name: String) {
        self.grade = Some(Box::new(Worker { role, name, next: self.grade.take() }))
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        match self.grade.take() {
            None => None,
            Some(worker) => {
                self.grade = worker.next;
                Some(worker.name)
            }
        }
        // if self.grade.is_none() {
        //     return None;
        // }
        // let name = self.grade.as_mut().unwrap().name.drain(..).collect();
        // self.grade = self.grade.as_mut().unwrap().next.take();
        // return Some(name);
    }

    pub fn last_worker(&self) -> Option<(String, String)> {
        match self.grade.as_ref() {
            None => None,
            Some(worker) => Some(
                (worker.name.to_owned(), worker.role.to_owned())
            ),
        }
        // if self.grade.is_none() {
        //     return None;
        // }
        // let w = self.grade.as_ref().unwrap();
        // Some((w.name.to_owned(), w.role.to_owned()))
    }
}

#[warn(dead_code)]
fn check_next_of_next(l: &Link) -> bool {
    !(l.is_none() || l.as_ref().unwrap().next.is_none() || l.as_ref().unwrap().next.as_ref().unwrap().next.is_none())
}

// pub fn remove_first_worker(&mut self) -> Option<String> {
//     let mut w: &mut Link = &mut self.grade;
//     if w.is_none() {
//         return None;
//     }
//     if w.as_ref().unwrap().next.is_none() {
//         let name = w.as_ref().unwrap().name.to_owned();
//         self.grade = None;
//         return Some(name);
//     }
//     loop {
//         if check_next_of_next(w) {
//             w = &mut w.as_mut().unwrap().next
//         } else {
//             let name = w.as_ref().unwrap().next.as_ref().unwrap().name.to_owned();
//             w.as_mut().unwrap().next = None;
//             break Some(name);
//         }
//     }
// }