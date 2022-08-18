use super::*;

pub(super) fn strct(p: &mut Parser) -> CompletedMarker {
    return build_struct(p);
}

fn build_struct(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.expect(TokenKind::LBrace);
    loop {
        if p.at(TokenKind::Octothorpe) {
            accessor::accessor(p);
    
        } else if p.at(TokenKind::SectionMarker) {
           build_section(p);
    
        } else if p.at(TokenKind::Ident) {
            record::build_standard(p);
    
        } else if p.at (TokenKind::DollarSign) {
            record::build_schema(p);
    
        } else if p.at(TokenKind::RAngleBrack) {
            record::build_anonymous(p);
    
        } else if p.at(TokenKind::LSquiggleArrow) {
            record::build_inlined(p);
    
        // TODO: NO.
        } else if p.at(TokenKind::Bang) {
            // Directive
            directive::directive(p);
        }
        else if p.at(TokenKind::RBrace) {
            p.bump();
            break;
        }
        else {
            p.error();
            break;
        }
    }
    return m.complete(p, SyntaxKind::Struct);
}


pub(super) fn build_section(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    p.expect(TokenKind::SectionMarker);

    
    if p.at(TokenKind::Octothorpe) {
        let n = p.start();
        ident::expand(p);
        p.if_bump(TokenKind::Colon);
        // p.expect(TokenKind::Colon);
        n.complete(p, SyntaxKind::Name);
    }
    else if p.at(TokenKind::At) {
        let n = p.start();
        ident::expand(p);
        p.if_bump(TokenKind::Colon);
        // p.expect(TokenKind::Colon);
        n.complete(p, SyntaxKind::Name);
    }
    else if p.at(TokenKind::LBird) {
        scope_stmt::expand(p);
    }

    if p.at(TokenKind::LBird) {
        scope_stmt::expand(p);
    }
    
    

    let close_brackets: bool = p.if_bump(TokenKind::LBrace);

    loop {
        println!(
            "top of loop"
        );
        if p.at(TokenKind::Octothorpe) {
            accessor::accessor(p);
    
        } else if p.at(TokenKind::SectionMarker) {
            if close_brackets {
                build_section(p);
            }
           else {
               break;
           }
        } else if p.at(TokenKind::Ident) {
            println!("building ident");
            record::build_standard(p);
    
        } else if p.at (TokenKind::DollarSign) {
            record::build_schema(p);
    
        } else if p.at(TokenKind::RAngleBrack) {
            record::build_anonymous(p);
    
        } else if p.at(TokenKind::LSquiggleArrow) {
            record::build_inlined(p);
    
        // TODO: NO.
        } else if p.at(TokenKind::Bang) {
            // Directive
            directive::directive(p);
        }
        else if p.at(TokenKind::RBrace) {
            if close_brackets {
                p.bump();
            }
            break;
        }
        else {
            p.error();
            break;
        }
    }

    return Some(m.complete(p, SyntaxKind::Section))
}