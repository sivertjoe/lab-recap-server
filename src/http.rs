pub trait Http
{
    fn get(&self, request: tiny_http::Request);
    fn post(&self, request: tiny_http::Request);
}
