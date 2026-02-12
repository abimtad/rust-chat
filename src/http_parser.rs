use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use http_muncher::ParserHandler;

pub struct HttpParser {
    current_key: Option<String>,
    headers: Rc<RefCell<HashMap<String, String>>>,
}

impl HttpParser {
    pub fn new(headers: Rc<RefCell<HashMap<String, String>>>) -> HttpParser {
        HttpParser {
            current_key: None,
            headers: headers,
        }
    }
}

impl ParserHandler for HttpParser {
    fn on_header_field(&mut self, s: &[u8]) -> bool {
        self.current_key = Some(std::str::from_utf8(s).unwrap().to_string());
        true
    }

    fn on_header_value(&mut self, s: &[u8]) -> bool {
        self.headers
            .borrow_mut()
            .insert(self.current_key.clone().unwrap(), std::str::from_utf8(s).unwrap().to_string());
        true
    }

    fn on_headers_complete(&mut self) -> bool {
        false
    }
}
