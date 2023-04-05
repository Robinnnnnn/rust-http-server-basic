
use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Debug, Formatter, write};
use std::str::{self, Utf8Error};

pub struct Request {
    path: &str,
    query_string: Option<&str>,
    method: Method, //super tells rust to go up one level to find the method module
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!()
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;


    /*
                        METHOD| PATH |Query Params
        Example Request: Get /search?name=abc&sort=1 HTTP/1.1
     */
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        
       let request = str::from_utf8(buf)?;
        
        // shadows and overwrites the current request variable
        // that way it returns only the second part of the string everytime
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method =  method.parse()?;
        let mut query_string:Option<&str>= None;

        // if Some() would return anything other than None, do something
        if let Some(i) = path.find('?') {
            query_string = Some(&path[i+1..]);
            path = &path[..i];
        }

        Ok(Self{
            path: path,
            query_string, //Some() is an option
            method,
        })
    }
 
}

//Gives me the first slice, and then the rest of string
fn get_next_word(request: &str) -> Option<(&str, &str)>{

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]));
        }
    }

    unimplemented!()
}
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {

}
