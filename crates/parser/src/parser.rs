pub(crate) mod marker;

mod parse_error;
pub(crate) use parse_error::ParseError;

use crate::event::Event;
use crate::grammar;
use crate::source::Source;
use lexer::{Token, TokenKind};
use marker::{Marker, CompletedMarker};
use std::mem;
use syntax::SyntaxKind;

// Fix this
const RECOVERY_SET: [TokenKind; 1] = [TokenKind::DotDotDot];
const LITERAL_TOKENS: [TokenKind; 8] = [
    TokenKind::HexLiteral,
    TokenKind::FloatLiteral,
    TokenKind::IntLiteral,
    TokenKind::OctalLiteral,
    TokenKind::BinaryLiteral,
    TokenKind::RandFloatLiteral,
    TokenKind::RandIntLiteral,
    TokenKind::String,
];

pub(crate) struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
    expected_kinds: Vec<TokenKind>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub(crate) fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
            expected_kinds: Vec::new(),
        }
    }

    pub(crate) fn parse(mut self) -> Vec<Event> {
        grammar::root(&mut self);
        self.events
    }

    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(pos)
    }

    /// Bump if token matches, else Error
    pub(crate) fn expect(&mut self, kind: TokenKind) {
        if self.at(kind) {
            self.bump();
        } else {
            self.error();
        }
    }

    /// Clears expected. Keeps the lines from crossing.
    pub(crate) fn clear_expected(&mut self) {
        self.expected_kinds.clear();
    }

    // TODO: I "Should" be able to back-climb and do a smart recovery based on core token sequences,
    //       which would add a ton of utility for the LSP. Rather than recover on 'let', as you would
    //       in rust, I'd have to have a set of context-specific recovery functions, recovering on :
    //       in records, on :: in accessors, terminating an unclosed function call at the start of
    //       the following block, etc.
    pub(crate) fn error(&mut self) {
        let current_token = self.source.peek_token();
        println!("Failed on: {:?}", current_token);

        let (found, range) = if let Some(Token { kind, range, .. }) = current_token {
            (Some(*kind), *range)
        } else {
            // If were at the end of the input we use the range of the very last token in the
            // input.
            (None, self.source.last_token_range().unwrap())
        };

        self.events.push(Event::Error(ParseError {
            expected: mem::take(&mut self.expected_kinds),
            found,
            range,
        }));

        // TODO: Figure out how to finesse this.
        if !self.at_set(&RECOVERY_SET) && !self.at_end() {
            let m = self.start();
            self.bump();
            m.complete(self, SyntaxKind::Error);
        }
    }

    /// Effectively error(), but with the theoretical ability to recover somehow.
    #[allow(dead_code)]
    pub(crate) fn smart_error(&mut self, failed_on: SyntaxKind) {
        let current_token = self.source.peek_token();
        println!("Failed on: {:?}", current_token);

        let (found, range) = if let Some(Token { kind, range, .. }) = current_token {
            (Some(*kind), *range)
        } else {
            // If were at the end of the input we use the range of the very last token in the
            // input.
            (None, self.source.last_token_range().unwrap())
        };

        self.events.push(Event::Error(ParseError {
            expected: mem::take(&mut self.expected_kinds),
            found,
            range,
        }));

        // Should I write error-recovery functions in each module, rather than here?
        // Then I could skip this match statement, call some stub function that just
        // does the above, and skip this whole match statement.
        match failed_on {
            SyntaxKind::Accessor => {},
            SyntaxKind::Directive => {},

            SyntaxKind::Expr => {},
            SyntaxKind::InfixExpr => {},
            SyntaxKind::PrefixExpr => {},
            SyntaxKind::ParenExpr => {},

            SyntaxKind::FuncCall => {},
            SyntaxKind::FuncArgs => {},

            SyntaxKind::Ident => {},

            SyntaxKind::List => {},

            SyntaxKind::ScopeBlock => {},
            SyntaxKind::Struct => {},
            SyntaxKind::Literal => {},
            SyntaxKind::Record => {},
            _ => println!("Failed in {:?}", failed_on)
        }

    }

    /// Advances the iterator and clears the expected token list
    pub(crate) fn bump(&mut self) {
        self.expected_kinds.clear();
        self.source.next_token().unwrap();
        self.events.push(Event::AddToken);
    }

    #[allow(dead_code)]
    pub(crate) fn get_at(&mut self) -> Option<TokenKind> {
        self.peek()
    }

    /// Check token and add it to list of expected tokens.
    /// Adds context information to error messages.
    pub(crate) fn at(&mut self, kind: TokenKind) -> bool {
        self.expected_kinds.push(kind);
        self.peek() == Some(kind)
    }

    pub(crate) fn at_next(&mut self, kind: TokenKind) -> bool {
        return self.source.peek_ahead() == Some(kind);
    }

    pub(crate) fn was_at(&mut self, kind: TokenKind) -> bool {
        return self.source.lookback() == Some(kind);
    }

    pub(crate) fn at_expandable(&mut self) -> bool {
        if let Some(curr) = self.peek() {
            return curr.is_prefix() || curr == TokenKind::Ident;
        }
        else {
            return false;
        }
    }

    pub(crate) fn at_op(&mut self) -> bool {
        if let Some(curr) = self.peek() {
            return curr.is_op()
        }
        return false;
    }

    #[allow(dead_code)]
    pub(crate) fn at_digraph(&mut self, pair: [TokenKind; 2]) -> bool {
        return self.source.peek_pair() == [Some(pair[0]), Some(pair[1])]
    }

    #[allow(dead_code)]
    pub(crate) fn auto_digraph(&mut self) -> SyntaxKind {
        let someBool = true;
        let example = 4 + if someBool {7} else {4} * 3;
        let exampleB = 4 + match someBool { true => 2, false => 5} * 3;
        let current = self.source.peek_pair();
        match [current[0].unwrap(), current[1].unwrap()] {
            [TokenKind::Colon, TokenKind::Colon]           => SyntaxKind::ColonColon,
            [TokenKind::Bang, TokenKind::Equals]           => SyntaxKind::NotEqual,
            [TokenKind::LAngleBrack, TokenKind::Equals]    => SyntaxKind::LessEqual,
            [TokenKind::RAngleBrack, TokenKind::Equals]    => SyntaxKind::GreaterEqual,
            [TokenKind::Tilde, TokenKind::Equals]          => SyntaxKind::PatternEqual,
            [TokenKind::Bang, TokenKind::Tilde]            => SyntaxKind::NotPattern,
            [TokenKind::Colon, TokenKind::Equals]          => SyntaxKind::Walrus,
            [TokenKind::Colon, TokenKind::QMark]           => SyntaxKind::Mustache,
            [TokenKind::LAngleBrack, TokenKind::Minus]     => SyntaxKind::LArrow,
            [TokenKind::Minus, TokenKind::RAngleBrack]     => SyntaxKind::RArrow,
            [TokenKind::LAngleBrack, TokenKind::Tilde]     => SyntaxKind::LSquiggleArrow,
            [TokenKind::Tilde, TokenKind::RAngleBrack]     => SyntaxKind::RSquiggleArrow,
            [TokenKind::LAngleBrack, TokenKind::Semicolon] => SyntaxKind::LBird,
            [TokenKind::Semicolon, TokenKind::Semicolon]   => SyntaxKind::RBird,
            [TokenKind::LAngleBrack, TokenKind::Carrot]    => SyntaxKind::LJuke,
            [TokenKind::Carrot, TokenKind::RAngleBrack]    => SyntaxKind::RJuke,
            _ => SyntaxKind::TombStone
        }
    }

    #[allow(dead_code)] // Bad?
    pub(crate) fn at_dec(&mut self) -> bool {
        self.peek() == Some(TokenKind::Ident)
            && self.source.peek_ahead() == Some(TokenKind::Colon)
    }

    pub(crate) fn at_literal(&mut self) -> bool {
        return self.at_set(&LITERAL_TOKENS);
    }

    // pub(crate) fn at_open(&mut self) -> bool {
    //     self.source.peek_n(3)
    //         .into_iter()
    //         .all(
    //             |t| t.map_or(false, |k| k == TokenKind::Dot))
    // }

    fn at_set(&mut self, set: &[TokenKind]) -> bool {
        self.peek().map_or(false, |k| set.contains(&k))
    }

    pub(crate) fn at_prefix(&mut self) -> bool {
        self.peek().map_or(
            false,
            |k| matches![
                k,
                TokenKind::DollarSign
                | TokenKind::Octothorpe
                | TokenKind::At])
    }
    
    pub(crate) fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    /// If at token, bump. Expects.
    pub(crate) fn if_bump(&mut self, kind: TokenKind) -> bool {
        // self.expected_kinds.push(kind);
        if self.at(kind) {
            self.bump();
            return true;
        } else {
            return false;
        }
    }

    pub(crate) fn peek_completed(&mut self, cm: CompletedMarker) {

    }

    // pub(crate) fn peek_literal(&mut self) -> String {
    //     return self.peek().unwrap_or(TokenKind::Error).to_string();
    // }

    fn peek(&mut self) -> Option<TokenKind> {
        self.source.peek_kind()
    }

}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_whitespace() {
        check(
            "   ",
            expect![[r#"
Root@0..3
  Whitespace@0..3 "   ""#]],
        );
    }

    #[test]
    fn parse_comment() {
        check(
            "// hello!",
            expect![[r##"
Root@0..9
  Comment@0..9 "// hello!""##]],
        );
    }

}
