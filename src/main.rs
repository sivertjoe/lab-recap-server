use serde_derive::Deserialize;

fn get_content(request: &mut tiny_http::Request) -> String
{
    let mut content = String::new();
    request.as_reader().read_to_string(&mut content).unwrap();
    content
}


#[derive(Deserialize)]
struct Comment
{
    comment: String,
}


fn main()
{
    let server = tiny_http::Server::http("0.0.0.0:5000").unwrap();
    let mut my_db_xd: Vec<String> = Vec::new();

    loop
    {
        println!("waiting..");
        let mut request = server.recv().unwrap();
        println!("{:?}", request);
        if request.url().starts_with("/comment")
        {
            match request.method()
            {
                &tiny_http::Method::Get =>
                {
                    let resp =
                        tiny_http::Response::from_string(format!("{{ comments: {:?}}}", my_db_xd));
                    request.respond(resp).expect("Responding");
                },

                &tiny_http::Method::Post =>
                {
                    let comment: Comment =
                        serde_json::from_str(&get_content(&mut request)).unwrap();
                    my_db_xd.push(comment.comment);
                    let resp = tiny_http::Response::empty(200);
                    request.respond(resp).expect("Responding");
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
