use lexer::{Token, TokenKind};
use text_size::TextRange;

pub(crate) struct Source<'t, 'input> {
    tokens: &'t [Token<'input>],
    cursor: usize,
}

impl<'t, 'input> Source<'t, 'input> {
    pub(crate) fn new(tokens: &'t [Token<'input>]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub(crate) fn next_token(&mut self) -> Option<&'t Token<'input>> {
        self.eat_trivia();

        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;

        Some(token)
    }

    pub(crate) fn peek_kind(&mut self) -> Option<TokenKind> {
        self.eat_trivia();
        self.peek_kind_raw()
    }

    pub(crate) fn peek_token(&mut self) -> Option<&Token> {
        self.eat_trivia();
        self.peek_token_raw()
    }

    pub(crate) fn lookback(&self) -> Option<TokenKind> {
        let mut n: usize = 0;
        while self.tokens.get(self.cursor - 1 - n)
            .map(|Token { kind, .. }| *kind)
                .map_or(false, TokenKind::is_trivia) == true {
                    n += 1;
        }
        return self.tokens.get(self.cursor - 1 - n).map(|Token { kind, .. }| *kind);
    }

    // TODO Do I need this?
    pub(crate) fn peek_pair(&mut self) -> [Option<TokenKind>; 2] {
        self.eat_trivia();
        let first = 
            self.tokens.get(self.cursor)
                .map(|Token { kind, .. }| *kind);

        let second = 
            self.tokens.get(self.cursor + 1)
                .map(|Token { kind, .. }| *kind);
        
        return [first, second];
    }

    // pub(crate) fn peek_n(&mut self, count: usize) -> Vec<Option<TokenKind>> {
    //     self.eat_trivia();
    //     let mut out_vec: Vec<Option<TokenKind>> = Vec::new();
    //     for i in 0..count {
    //         out_vec.push(
    //             self.tokens.get(self.cursor + i)
    //                 .map(|Token { kind, .. }| *kind)
    //         );
    //     }
    //     return out_vec;
    // }

    pub(crate) fn peek_ahead(&mut self) -> Option<TokenKind> {
        self.eat_trivia();
        self.tokens.get(self.cursor + self.skip_trivia() + 1)
                .map(|Token { kind, .. }| *kind)
    }



    fn eat_trivia(&mut self) {
        while self.at_trivia() {
            self.cursor += 1;
        }
    }

    fn skip_trivia(&mut self) -> usize {
        let mut n: usize = 0;
        while self.tokens.get(self.cursor + n)
            .map(|Token { kind, .. }| *kind)
                .map_or(false, TokenKind::is_trivia) == true {
                    n += 1;
        }
        return n;
    }

    fn at_trivia(&self) -> bool {
        self.peek_kind_raw().map_or(false, TokenKind::is_trivia)
    }

    pub(crate) fn last_token_range(&self) -> Option<TextRange> {
        self.tokens.last().map(|Token { range, .. }| *range)
    }

    fn peek_kind_raw(&self) -> Option<TokenKind> {
        self.peek_token_raw().map(|Token { kind, .. }| *kind)
    }

    fn peek_token_raw(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }


}