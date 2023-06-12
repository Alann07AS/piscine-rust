pub struct Message {
    pub content: String,
    pub user: String
}

impl Message {
    pub fn new(content: String, user: String) -> Message {
        Message {
            content: content,
            user: user
        }
    }
    pub fn send_ms(&self) -> Option<&str> {
        let str = self.content.as_str(); 
        if str == "" || str.contains("stupid")  {
            None
        } else {
            Some(str)
        }
    }
}

pub fn check_ms(ms: &Message) -> (bool, &str) {
    match ms.send_ms() {
        None => (false, "ERROR: illegal"),
        Some(s) => (true, s)
    }
}