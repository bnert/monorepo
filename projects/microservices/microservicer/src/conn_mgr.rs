use std::{collections, thread};

pub struct ConnMgr {
    conn: collections::HashMap<String, thread::JoinHandle<()>>
}

impl ConnMgr {
    fn new() -> ConnMgr {
        ConnMgr{
            conn: collections::HashMap::new()
        }
    }

    fn conn_name_exists(&self, handle_name: &String) -> bool {
        self.conn.contains_key(handle_name)
    }

    fn handle_conn(&mut self, handle_name: &String) -> &ConnMgr {
        // handles a connection
        self.conn.insert(handle_name.to_string(), thread::spawn(move || {
                for i in 0..9 {
                    println!("{}", i)
                }
            })
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiation() {
        let mut cm = ConnMgr::new();
        let cm = cm.handle_conn(&"handle".to_string());
        assert_eq!(cm.conn_name_exists(&"handle".to_string()), true);
    }
}
