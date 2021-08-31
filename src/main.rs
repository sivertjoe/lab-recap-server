mod client;
mod http;
mod server;

#[macro_use] extern crate dotenv_codegen;

use dotenv::dotenv;
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVER: String = {
        dotenv().ok();
        dotenv!("SERVER").to_string()
    };

    static ref CLIENT: String = {
        dotenv().ok();
        dotenv!("CLIENT").to_string()
    };
}

fn main()
{
    let (handler, addr): (Box<dyn http::Http>, &String) =
        match std::env::args().nth(1).unwrap().as_str()
        {
            "c" => (Box::new(client::Client), &*CLIENT),
            "s" => (Box::new(server::Server::new()), &*SERVER),
            _ => panic!("pick either 's' or 'c'"),
        };
    server(handler, addr);
}

fn server(handler: Box<dyn http::Http>, addr: &String)
{
    let server = tiny_http::Server::http(addr).unwrap();
    loop
    {
        println!("waiting..");
        let request = server.recv().unwrap();
        println!("{:?}", request);
        if request.url().starts_with("/comment")
        {
            match request.method()
            {
                &tiny_http::Method::Get =>
                {
                    handler.get(request);
                },

                &tiny_http::Method::Post =>
                {
                    handler.post(request);
                },
                _ =>
                {
                    let resp = tiny_http::Response::empty(401);
                    request.respond(resp).expect("Responding");
                },
            };
        }
    }
}
