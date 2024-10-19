mod token_stream;
mod token;

pub type Token = token::Token;
pub type TokenPos = token::TokenPos;
pub type TokenVal = token::TokenVal;
pub type TokenStream<'a> = token_stream::TokenStream<'a>;