
use super::method::Method;
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method, //super tells rust to go up one level to find the method module
}
