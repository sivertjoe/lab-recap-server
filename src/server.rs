use std::cell::RefCell;

use serde_derive::{Deserialize, Serialize};

use crate::http::Http;

#[derive(Deserialize, Serialize)]
struct Comment
{
    comment: String,
}
pub struct Server
{
    db_xd: RefCell<Vec<String>>,
}

impl Server
{
    pub fn new() -> Self
    {
        Server {
            db_xd: RefCell::new(Vec::new())
        }
    }
}

pub fn get_content(request: &mut tiny_http::Request) -> String
{
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content).unwrap();
    content
}

impl Http for Server
{
    fn get(&self, request: tiny_http::Request)
    {
        let resp = tiny_http::Response::from_string(format!(
            "{{ \"comments\": {:?}}}",
            self.db_xd.borrow()
        ));
        request.respond(resp).expect("Responding");
    }

    fn post(&self, mut request: tiny_http::Request)
    {
        let comment: Comment = serde_json::from_str(&get_content(&mut request)).unwrap();

        let mut db = self.db_xd.borrow_mut();
        db.push(comment.comment);

        let resp = tiny_http::Response::empty(200);
        request.respond(resp).expect("Responding");
    }
}
