use super::*;

/// expects an identifier, constructs a path.
pub(super) fn expand(p: &mut Parser) -> CompletedMarker {
    return path(p, true);
}

fn path(p: &mut Parser, allow_prefix: bool) -> CompletedMarker {
    let mut m = p.start();

    if p.at_prefix() {
        if allow_prefix {
            p.bump();
        } else {
            println!("Unexpected Prefix!");
            p.error();
        }
    }

    let cm = if p.at(TokenKind::Ident) {
        p.bump();
        m = m.complete(p, SyntaxKind::Name).precede(p);
        path_sep(p);
        m.complete(p, SyntaxKind::Path)
    }
 
    else if p.at(TokenKind::LBrack) {
        expr::expr(p);
        m.complete(p, SyntaxKind::FilterExpr)
    }

    else {
        println!("left the member open");
        m.complete(p, SyntaxKind::LJuke)
    };

    return cm;
}

fn path_sep(p: &mut Parser) {
    
    if p.at(TokenKind::Dot) {
        p.bump();
        println!("recurse on dot");
        path(p, true);
    }

    else if p.at(TokenKind::ColonColon) {
        println!("recurse on colon");
        p.bump();
        path(p, true);
    }

    // Squiggle Arrow ≠ Composition Operator
    else if p.at(TokenKind::RSquiggleArrow) {
        let m = p.start();
        p.bump();
        path(p, false);
        // m.complete(p, SyntaxKind::Path)
        m.complete(p, SyntaxKind::FuncCall);
    }

    // Non Terminating
    else if p.at(TokenKind::LParen) {
        // p.bump();
        func::func_args(p);
        path_sep(p);
        // return None;
    }

    // Terminating
    else if p.at(TokenKind::LBrack) {
        let m = p.start();
        p.bump();
        expr::expr(p);
        p.expect(TokenKind::RBrack);
        m.complete(p, SyntaxKind::TypePrimitive);

    }
    
    // Terminating
    else {
        println!("terminate on else");

    };

}


#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;
    
    #[test]
    fn parse_simple_ident() {
        check(
r#"jim: "jimmy""#,
expect![[
r##"
Root@0..12
  Record@0..12
    Name@0..5
      Ident@0..3 "jim"
      Colon@3..4 ":"
      Whitespace@4..5 " "
    Body@5..12
      Literal@5..12
        String@5..12 "\"jimmy\"""##]]
        )
    }

    #[test]
    fn parse_accessor_ref() {
        check(
r#"jim: jimmy::jimbo.john"#,
expect![[
r##"
Root@0..22
  Record@0..22
    Name@0..5
      Ident@0..3 "jim"
      Colon@3..4 ":"
      Whitespace@4..5 " "
    Body@5..22
      Ref@5..22
        Path@5..22
          Name@5..10
            Ident@5..10 "jimmy"
          ColonColon@10..12 "::"
          Path@12..22
            Name@12..17
              Ident@12..17 "jimbo"
            Dot@17..18 "."
            Path@18..22
              Name@18..22
                Ident@18..22 "john""##]]
        )
    }

    #[test]
    fn parse_member_ref() {
        check(
r#"jim: jimmy.jimbo"#,
expect![[
r##"
Root@0..16
  Record@0..16
    Name@0..5
      Ident@0..3 "jim"
      Colon@3..4 ":"
      Whitespace@4..5 " "
    Body@5..16
      Ref@5..16
        Path@5..16
          Name@5..10
            Ident@5..10 "jimmy"
          Dot@10..11 "."
          Path@11..16
            Name@11..16
              Ident@11..16 "jimbo""##]]
        )
    }

    #[test]
    fn parse_filtered_ref() {
        check(
r#"jim: jimmy::jimbo.[name, age]"#,
expect![[
r##"
Root@0..29
  Record@0..29
    Name@0..5
      Ident@0..3 "jim"
      Colon@3..4 ":"
      Whitespace@4..5 " "
    Body@5..29
      Ref@5..29
        Path@5..29
          Name@5..10
            Ident@5..10 "jimmy"
          ColonColon@10..12 "::"
          Path@12..29
            Name@12..17
              Ident@12..17 "jimbo"
            Dot@17..18 "."
            FilterExpr@18..29
              List@18..29
                LBrack@18..19 "["
                Ref@19..23
                  Path@19..23
                    Name@19..23
                      Ident@19..23 "name"
                Comma@23..24 ","
                Whitespace@24..25 " "
                Ref@25..28
                  Path@25..28
                    Name@25..28
                      Ident@25..28 "age"
                RBrack@28..29 "]""##]]
        )
    }

    #[test]
    fn parse_function_chain() {
        check(
r#"jim: jimmy~>filter()~>reduce()"#,
expect![[
r##"
Root@0..30
  Record@0..30
    Name@0..5
      Ident@0..3 "jim"
      Colon@3..4 ":"
      Whitespace@4..5 " "
    Body@5..30
      Ref@5..30
        Path@5..30
          Name@5..10
            Ident@5..10 "jimmy"
          FuncCall@10..30
            RSquiggleArrow@10..12 "~>"
            Path@12..30
              Name@12..18
                Ident@12..18 "filter"
              FuncArgs@18..20
                LParen@18..19 "("
                RParen@19..20 ")"
              FuncCall@20..30
                RSquiggleArrow@20..22 "~>"
                Path@22..30
                  Name@22..28
                    Ident@22..28 "reduce"
                  FuncArgs@28..30
                    LParen@28..29 "("
                    RParen@29..30 ")""##]]
        )
    }
}