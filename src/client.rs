use std::collections::HashMap;

use reqwest::blocking::Client as Cl;

use crate::{http::Http, SERVER};

pub struct Client;

impl Http for Client
{
    fn get(&self, request: tiny_http::Request)
    {
        let comments = Cl::new()
            .get(&format!("http://{}/comment", &*SERVER))
            .send()
            .unwrap()
            .json::<HashMap<String, Vec<String>>>()
            .map(|mut map| map.remove("comments").unwrap())
            .unwrap();

        let resp = tiny_http::Response::from_string(format!("{{ comments: {:?}}}", comments));
        request.respond(resp).expect("Responding");
    }

    fn post(&self, mut request: tiny_http::Request)
    {
        let comment = crate::server::get_content(&mut request).bytes().collect::<Vec<u8>>();
        Cl::new()
            .post(&format!("http://{}/comment", &*SERVER))
            .body(comment)
            .send()
            .unwrap();
        let resp = tiny_http::Response::empty(200);
        request.respond(resp).expect("Responding");
    }
}
