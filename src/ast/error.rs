use crate::token_stream::TokenPos;

pub struct Error<'a, 'b> {
    source_plain: &'a str,
    pos: TokenPos,
    msg: ErrorMsg<'b>,
}

pub enum ErrorMsg<'a> {
    Unexpected{ want: &'a str }
}

impl<'a, 'b> Error<'a, 'b> {
    pub fn new(source_plain: &'a str, pos: TokenPos, msg: ErrorMsg<'b>) -> Self {
        Self { source_plain, pos, msg }
    }

    /// Print the error to stderr.
    pub fn print(&self) {
        let lines = self.source_plain.lines();
        let mut lineno = 1;
        for line in lines {
            eprintln!("{}", line);
            if lineno == self.pos.lineno {
                for _ in 0..(self.pos.offset - 1) {
                    // TODO (@PeterlitsZo) We guess 1 character's width == 1
                    // space's width. But the CJK / '\t' character are not.
                    eprint!(" ");
                }
                eprint!("^ ");
                match self.msg {
                    ErrorMsg::Unexpected { want } => {
                        eprintln!("want {}.", want)
                    }
                }
            }
            lineno += 1;
        }
    }
}