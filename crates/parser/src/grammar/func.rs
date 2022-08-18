use super::*;

pub(super) fn func_args(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    p.expect(TokenKind::LParen);
    loop {
        p.clear_expected();
        if p.if_bump(TokenKind::Comma) {
            println!("func args: at comma");
            expr::expr(p);
        } else if p.at(TokenKind::Ident) {
            println!("func args: at ident");
            expr::expr(p);
        } else if p.at_literal(){
            println!("func_args_at_literal");
            expr::expr(p);
        } else if p.at(TokenKind::DollarSign) {
            // Technically valid

            if p.at_next(TokenKind::Ident) {
              ident::expand(p);
            } else {
              p.bump();
            }
            
        } else if p.if_bump(TokenKind::At) {
          accessor::accessor_ref(p);
        } else if p.if_bump(TokenKind::Octothorpe) {
            // I need to handle these better 
        } else if p.if_bump(TokenKind::RParen) {
            break;
        } else {
            println!("four");
            p.error();
            break;
        }
    }
    return Some(m.complete(p, SyntaxKind::FuncArgs));
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_func_min() {
        check(
            "<~ do()",
            expect![[
                r#"
Root@0..7
  InlinedRecord@0..7
    LSquiggleArrow@0..2 "<~"
    Whitespace@2..3 " "
    Path@3..7
      Name@3..5
        Ident@3..5 "do"
      FuncArgs@5..7
        LParen@5..6 "("
        RParen@6..7 ")""#
            ]]
        )
    }

    #[test]
    fn parse_func_missing_paren() {
        check(
            "<~ do(a + 4",
            expect![[
                r#"
Root@0..11
  InlinedRecord@0..11
    LSquiggleArrow@0..2 "<~"
    Whitespace@2..3 " "
    Path@3..11
      Name@3..5
        Ident@3..5 "do"
      FuncArgs@5..11
        LParen@5..6 "("
        InfixExpr@6..11
          Ref@6..8
            Path@6..8
              Name@6..8
                Ident@6..7 "a"
                Whitespace@7..8 " "
          Plus@8..9 "+"
          Whitespace@9..10 " "
          Literal@10..11
            Number@10..11 "4"
error at 10..11: expected ,, identifier, $, @, # or )"#
            ]]
        )
    }

    #[test]
    fn parse_func_call_no_args() {
        check(
            "<~ thing.do()",
            expect![[
                r#"
Root@0..13
  InlinedRecord@0..13
    LSquiggleArrow@0..2 "<~"
    Whitespace@2..3 " "
    Path@3..13
      Name@3..8
        Ident@3..8 "thing"
      Dot@8..9 "."
      Path@9..13
        Name@9..11
          Ident@9..11 "do"
        FuncArgs@11..13
          LParen@11..12 "("
          RParen@12..13 ")""#
            ]]
        )
    }

    #[test]
    fn parse_func_call_token_arg() {
        check(
            "<~ thing.do($)",
            expect![[
                r#"
Root@0..14
  InlinedRecord@0..14
    LSquiggleArrow@0..2 "<~"
    Whitespace@2..3 " "
    Path@3..14
      Name@3..8
        Ident@3..8 "thing"
      Dot@8..9 "."
      Path@9..14
        Name@9..11
          Ident@9..11 "do"
        FuncArgs@11..14
          LParen@11..12 "("
          DollarSign@12..13 "$"
          RParen@13..14 ")""#
            ]]
        )
    }

    #[test]
    fn parse_func_call_single_arg() {
        check(
            "<~ thing.do(test)",
            expect![[
                r#"
Root@0..17
  InlinedRecord@0..17
    LSquiggleArrow@0..2 "<~"
    Whitespace@2..3 " "
    Path@3..17
      Name@3..8
        Ident@3..8 "thing"
      Dot@8..9 "."
      Path@9..17
        Name@9..11
          Ident@9..11 "do"
        FuncArgs@11..17
          LParen@11..12 "("
          Ref@12..16
            Path@12..16
              Name@12..16
                Ident@12..16 "test"
          RParen@16..17 ")""#
            ]]
        )
    }

    #[test]
    fn parse_func_call_multiple_args() {
        check(
            "<~ thing.do(test, more_test, 4+4)",
            expect![[
                r#"
Root@0..33
  InlinedRecord@0..33
    LSquiggleArrow@0..2 "<~"
    Whitespace@2..3 " "
    Path@3..33
      Name@3..8
        Ident@3..8 "thing"
      Dot@8..9 "."
      Path@9..33
        Name@9..11
          Ident@9..11 "do"
        FuncArgs@11..33
          LParen@11..12 "("
          Ref@12..16
            Path@12..16
              Name@12..16
                Ident@12..16 "test"
          Comma@16..17 ","
          Whitespace@17..18 " "
          Ref@18..27
            Path@18..27
              Name@18..27
                Ident@18..27 "more_test"
          Comma@27..28 ","
          Whitespace@28..29 " "
          InfixExpr@29..32
            Literal@29..30
              Number@29..30 "4"
            Plus@30..31 "+"
            Literal@31..32
              Number@31..32 "4"
          RParen@32..33 ")""#
            ]]
        )
    }

    // TODO: Is this really the way I want to handle this?
    #[test]
    fn parse_member_func_call() {
        check(
            r#"<~ parent.child.do(test, "more_test", 4+4)"#,
            expect![[
                r#"
Root@0..42
  InlinedRecord@0..42
    LSquiggleArrow@0..2 "<~"
    Whitespace@2..3 " "
    Path@3..42
      Name@3..9
        Ident@3..9 "parent"
      Dot@9..10 "."
      Path@10..42
        Name@10..15
          Ident@10..15 "child"
        Dot@15..16 "."
        Path@16..42
          Name@16..18
            Ident@16..18 "do"
          FuncArgs@18..42
            LParen@18..19 "("
            Ref@19..23
              Path@19..23
                Name@19..23
                  Ident@19..23 "test"
            Comma@23..24 ","
            Whitespace@24..25 " "
            Literal@25..36
              String@25..36 "\"more_test\""
            Comma@36..37 ","
            Whitespace@37..38 " "
            InfixExpr@38..41
              Literal@38..39
                Number@38..39 "4"
              Plus@39..40 "+"
              Literal@40..41
                Number@40..41 "4"
            RParen@41..42 ")""#
            ]]
        )
    }
}