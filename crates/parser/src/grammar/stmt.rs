use super::*;

pub(super) fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    return build_stmt(p);
}

fn build_stmt(p: &mut Parser) -> Option<CompletedMarker> {
    let cm = 
    if p.at_expandable() {
        expand::expand(p, true)
    }
    else if p.at(TokenKind::SectionMarker) {
        strct::build_section(p)
    }
    else if p.at(TokenKind::RAngleBrack) {        // TODO: Ambigious
        record::build_anonymous(p)
    } 
    else if p.at(TokenKind::LSquiggleArrow) {
        record::build_inlined(p)

    } else if p.at_literal() {
        println!("MISSING <");
        p.error();
        expr::expr(p)
    } else if p.at(TokenKind::Minus) {
        println!("MISSING <");
        p.error();
        expr::expr(p)
    } else if p.at(TokenKind::LParen) {
        println!("MISSING <");
        p.error();
        expr::expr(p)
    } else {
        println!("UNHANDLED CASE");
        p.error();
        expr::expr(p)
    };

    return cm;
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_variable_definition() {
        check(
            "foo : bar",
            expect![[r#"
Root@0..9
  Record@0..9
    Name@0..4
      Ident@0..3 "foo"
      Whitespace@3..4 " "
    Colon@4..5 ":"
    Whitespace@5..6 " "
    Body@6..9
      Name@6..9
        Ident@6..9 "bar""#]],
        );
    }

    #[test]
    fn parse_member_assignment() {
        check(
            "foo.bar.baz : boo",
            expect![[r#"
Root@0..17
  Record@0..17
    Path@0..12
      Name@0..3
        Ident@0..3 "foo"
      Dot@3..4 "."
      Path@4..12
        Name@4..7
          Ident@4..7 "bar"
        Dot@7..8 "."
        Name@8..12
          Ident@8..11 "baz"
          Whitespace@11..12 " "
    Colon@12..13 ":"
    Whitespace@13..14 " "
    Body@14..17
      Name@14..17
        Ident@14..17 "boo""#]],
        );
    }

    // TODO: Write proper test for circular declaration
    #[test]
    fn recover_on_bad_dec() {
        check(
            "a:\nb: a",
            expect![[r#"
Root@0..7
  Record@0..7
    Name@0..3
      Ident@0..1 "a"
      Colon@1..2 ":"
      Whitespace@2..3 "\n"
    Body@3..7
      InfixExpr@3..7
        Ref@3..4
          Path@3..4
            Name@3..4
              Ident@3..4 "b"
        Colon@4..5 ":"
        Whitespace@5..6 " "
        Ref@6..7
          Path@6..7
            Name@6..7
              Ident@6..7 "a""#]],
        );
    }
}
