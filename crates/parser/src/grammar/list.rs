use super::*;

// Only used by scope_stmt::expand, should be depreciated/inlined
pub(super) fn expand(p: &mut Parser, open_on: TokenKind, close_on: TokenKind) -> Option<CompletedMarker> {
    let m = p.start();
    p.expect(open_on);
    loop {

        if p.at(TokenKind::Comma) {
            p.bump();
        }

        if p.at(close_on) {
            p.bump();
            break;
        }

        if p.at_end() {
            break;
        }

        expr::expr(p);
    }

    return Some(m.complete(p, SyntaxKind::List));
}



