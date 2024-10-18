mod token_stream;
mod token;

pub type Token = token::Token;
pub type TokenStream<'a> = token_stream::TokenStream<'a>;