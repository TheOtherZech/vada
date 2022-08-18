use super::*;

pub(super) fn expand(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    // p.expect(TokenKind::LBird);
    list::expand(p, TokenKind::LBird, TokenKind::RBird);
    // p.expect(TokenKind::RBird);

    return Some(m.complete(p, SyntaxKind::ScopeBlock)); // 
}