use super::*;

pub(super) fn directive(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(TokenKind::Bang));
    if p.at_next(TokenKind::LBrack) {
        return match_directive(p);
    }

    else if p.at_next(TokenKind::Ident) {
        return keyword_directive(p);
    }
    
    else {
        p.error();
        return None;
    }
}

fn keyword_directive(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    p.expect(TokenKind::Bang);
    p.expect(TokenKind::Ident);

    if p.at(TokenKind::LParen) {
        func::func_args(p);
        return Some(m.complete(p, SyntaxKind::Directive));
    }

    else if p.at(TokenKind::LBrace) {
        strct::strct(p);
        return Some(m.complete(p, SyntaxKind::TypePrimitive));
    }

    else if p.at(TokenKind::LBrack) {
        p.bump();

        if p.at_literal() {
            expr::expr(p);
        } else {
            expr::expr(p);
        }
        p.expect(TokenKind::RBrack);
        return Some(m.complete(p, SyntaxKind::TypePrimitive))
    } else {
        println!("Exiting primitive dec");
        // p.expect(TokenKind::RBrack);
        return Some(m.complete(p, SyntaxKind::TypePrimitive))
    }
    
}

fn match_directive(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    p.expect(TokenKind::Bang);
    p.expect(TokenKind::LBrack);

    if p.at(TokenKind::At) {
        // Match Set
    }
    else if p.at(TokenKind::String) {
        // Match Name
    }
    else if p.at(TokenKind::RBrace) {
        // Empty Match
    }
    else {
        // Context Constraint
        expr::expr(p);
    }

    println!("herhehre");
    p.expect(TokenKind::RBrack);

    return Some(m.complete(p, SyntaxKind::Directive));

}


#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_enum_dec() {
        check(
r#"!enum{
    --- #flags 
    A: 1 .> 1
    B: 1 .> 2
    --- #ranges
    prim: @self.A..@self.D
    --- #groups
    AB: A & B
}"#,
expect![[
r##"
Root@0..125
  TypePrimitive@0..125
    Bang@0..1 "!"
    Ident@1..5 "enum"
    Struct@5..125
      LBrace@5..6 "{"
      Whitespace@6..11 "\n    "
      Section@11..55
        SectionMarker@11..14 "---"
        Whitespace@14..15 " "
        Name@15..27
          Path@15..27
            Name@15..27
              Octothorpe@15..16 "#"
              Ident@16..21 "flags"
              Whitespace@21..27 " \n    "
        Record@27..41
          Name@27..30
            Ident@27..28 "A"
            Colon@28..29 ":"
            Whitespace@29..30 " "
          Body@30..41
            InfixExpr@30..41
              Literal@30..32
                Number@30..31 "1"
                Whitespace@31..32 " "
              BitShiftRight@32..34 ".>"
              Whitespace@34..35 " "
              Literal@35..41
                Number@35..36 "1"
                Whitespace@36..41 "\n    "
        Record@41..55
          Name@41..44
            Ident@41..42 "B"
            Colon@42..43 ":"
            Whitespace@43..44 " "
          Body@44..55
            InfixExpr@44..55
              Literal@44..46
                Number@44..45 "1"
                Whitespace@45..46 " "
              BitShiftRight@46..48 ".>"
              Whitespace@48..49 " "
              Literal@49..55
                Number@49..50 "2"
                Whitespace@50..55 "\n    "
      Section@55..98
        SectionMarker@55..58 "---"
        Whitespace@58..59 " "
        Name@59..71
          Path@59..71
            Name@59..71
              Octothorpe@59..60 "#"
              Ident@60..66 "ranges"
              Whitespace@66..71 "\n    "
        Record@71..98
          Name@71..77
            Ident@71..75 "prim"
            Colon@75..76 ":"
            Whitespace@76..77 " "
          Body@77..98
            InfixExpr@77..98
              Ref@77..84
                Path@77..84
                  Name@77..82
                    At@77..78 "@"
                    Ident@78..82 "self"
                  Dot@82..83 "."
                  Path@83..84
                    Name@83..84
                      Ident@83..84 "A"
              DotDot@84..86 ".."
              Ref@86..98
                Path@86..98
                  Name@86..91
                    At@86..87 "@"
                    Ident@87..91 "self"
                  Dot@91..92 "."
                  Path@92..98
                    Name@92..98
                      Ident@92..93 "D"
                      Whitespace@93..98 "\n    "
      Section@98..124
        SectionMarker@98..101 "---"
        Whitespace@101..102 " "
        Name@102..114
          Path@102..114
            Name@102..114
              Octothorpe@102..103 "#"
              Ident@103..109 "groups"
              Whitespace@109..114 "\n    "
        Record@114..124
          Name@114..118
            Ident@114..116 "AB"
            Colon@116..117 ":"
            Whitespace@117..118 " "
          Body@118..124
            InfixExpr@118..124
              Ref@118..120
                Path@118..120
                  Name@118..120
                    Ident@118..119 "A"
                    Whitespace@119..120 " "
              Ampersand@120..121 "&"
              Whitespace@121..122 " "
              Ref@122..124
                Path@122..124
                  Name@122..124
                    Ident@122..123 "B"
                    Whitespace@123..124 "\n"
      RBrace@124..125 "}""##
]]
        )
    }

    // TODO: This needs a proper body on it.
    #[test]
    fn parse_scalar_type_dec() {
        check(
            "!scalar[!int & >=0]",
            expect![[
r##"
Root@0..19
  TypePrimitive@0..19
    Bang@0..1 "!"
    Ident@1..7 "scalar"
    LBrack@7..8 "["
    InfixExpr@8..18
      TypePrimitive@8..13
        Bang@8..9 "!"
        Ident@9..12 "int"
        Whitespace@12..13 " "
      Ampersand@13..14 "&"
      Whitespace@14..15 " "
      PrefixExpr@15..18
        GreaterEqual@15..17 ">="
        Literal@17..18
          Number@17..18 "0"
    RBrack@18..19 "]""##]]
        )
    }

    #[test]
    fn parse_vector_type_dec() {
        check(
            "!vec[ 3 -> $uint ]",
            expect![[
r##"
Root@0..18
  TypePrimitive@0..18
    Bang@0..1 "!"
    Ident@1..4 "vec"
    LBrack@4..5 "["
    Whitespace@5..6 " "
    InfixExpr@6..17
      Literal@6..8
        Number@6..7 "3"
        Whitespace@7..8 " "
      RArrow@8..10 "->"
      Whitespace@10..11 " "
      Type@11..17
        Path@11..17
          Name@11..17
            DollarSign@11..12 "$"
            Ident@12..16 "uint"
            Whitespace@16..17 " "
    RBrack@17..18 "]""##]]
        )
    }


}