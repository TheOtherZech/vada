use super::*;

pub(super) fn expand(p: &mut Parser, do_statement: bool) -> Option<CompletedMarker> {
    if do_statement {
        let lhs = exp(p);
        if p.at(TokenKind::Colon) || p.at(TokenKind::LSquiggleArrow) {
            let outer = lhs.precede(p);
            p.bump();
            let m = p.start();
            expr::expr(p);
            m.complete(p, SyntaxKind::Body);
            return Some(outer.complete(p, SyntaxKind::Record));
        }
        else {
            return Some(lhs);
        }
    } 
    else {
        return Some(exp(p));
    } 
}

fn exp(p: &mut Parser) -> CompletedMarker {
    let cm = 
    if p.at(TokenKind::Bang) {  // Type Primitive
        let outer = p.start();
        let inner = p.start();
        p.bump();
        p.expect(TokenKind::Ident);
        inner.complete(p, SyntaxKind::Name);
        if p.at(TokenKind::LBrace) {
            strct::strct(p);
        }
        outer.complete(p, SyntaxKind::Type)
    }

    else if p.at(TokenKind::At) { // Keyword
        let m = p.start(); 
        p.bump();
        if p.at(TokenKind::Ident) {
            p.bump();
            m.complete(p, SyntaxKind::Keyword)

        }
        else if p.at(TokenKind::LBrack) { // Dynamic Field
            expr::expr(p);
            m.complete(p, SyntaxKind::Directive) // Is this the right SK?
        }
        else {
            p.error();
            m.complete(p, SyntaxKind::Error)
        }
    }

    else if p.at(TokenKind::Octothorpe) { // Transform
        let outer = p.start();
        let inner = p.start();
        p.bump();
        p.expect(TokenKind::Ident);
        inner.complete(p, SyntaxKind::Name);
        if p.at(TokenKind::LBrace) {
            strct::strct(p);
        }
        println!("octo'd");
        outer.complete(p, SyntaxKind::Transform)
    }

    else if p.at(TokenKind::DollarSign) { // Schema
        let outer = p.start();
        let inner = p.start();
        p.bump();
        p.expect(TokenKind::Ident);
        inner.complete(p, SyntaxKind::Name);
        if p.at(TokenKind::LBrace) {
            strct::strct(p);
        }
        outer.complete(p, SyntaxKind::Schema)
    }

    else if p.at(TokenKind::Ident) {
        let m = p.start(); // Generic
        p.bump();
        m.complete(p, SyntaxKind::Name)
    }

    else if p.at(TokenKind::LBrack) {
        let m = p.start(); // Filter expression
        expr::expr(p);
        m.complete(p, SyntaxKind::FilterExpr)
    }
    else {
        unreachable!()
    };

    return cm;

    if p.at(TokenKind::Dot) ||  p.at(TokenKind::ColonColon) {
        let outer = cm.precede(p);
        p.bump();
        exp(p);
        return outer.complete(p, SyntaxKind::Path);
    }
    else {
        println!("exit expansion");
        return cm;
    }

}


