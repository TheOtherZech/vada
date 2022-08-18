/*
TL;DR: Accessors are import statements on steroids.

       ┌─ Alias
       │         ┌─ Path/scope separator
       │         │               ┌─ Simple type filter
       ↓         ↓               ↓
use |vtype| @arc::coretypes.vecs($)
            ↑              ↑
            │              └─ Manifest member accessor
            └─ Root identifier


                      ┌─ Root nesting is permitted
                      │          ┌─ Alters the accessor's interface by mapping the source
                      │          │  collection's manifest to the struct after the arrow.
                      ↓          ↓
use |therim| @parent::@siblings -> {
    entities: therim.entities.filter(!!implements($throgdor))
    factions: therim.factions.major
}
*/

use super::*;

pub(super) fn accessor_ref(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    p.expect(TokenKind::Octothorpe);
    ident::expand(p);

    return Some(m.complete(p, SyntaxKind::Accessor));
}

pub(super) fn accessor(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();

    let name = p.start();
    p.expect(TokenKind::Octothorpe);
    p.expect(TokenKind::Ident);
    p.expect(TokenKind::Colon);
    name.complete(p, SyntaxKind::Name);

    ident::expand(p);

    return Some(m.complete(p, SyntaxKind::Accessor));
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_simple_accessor() {
        check(
            "#zalgo: @global::zalgo",
            expect![[r##"
Root@0..22
  Accessor@0..22
    Name@0..8
      Octothorpe@0..1 "#"
      Ident@1..6 "zalgo"
      Colon@6..7 ":"
      Whitespace@7..8 " "
    Path@8..22
      Name@8..15
        At@8..9 "@"
        Ident@9..15 "global"
      ColonColon@15..17 "::"
      Path@17..22
        Name@17..22
          Ident@17..22 "zalgo""##]],
        )
    }

    #[test]
    fn parse_pathed_import() {
        check(
            "#z_mixins: @local::zalgo::tools",
            expect![[r##"
Root@0..31
  Accessor@0..31
    Name@0..11
      Octothorpe@0..1 "#"
      Ident@1..9 "z_mixins"
      Colon@9..10 ":"
      Whitespace@10..11 " "
    Path@11..31
      Name@11..17
        At@11..12 "@"
        Ident@12..17 "local"
      ColonColon@17..19 "::"
      Path@19..31
        Name@19..24
          Ident@19..24 "zalgo"
        ColonColon@24..26 "::"
        Path@26..31
          Name@26..31
            Ident@26..31 "tools""##]],
        )
    }

    #[test]
    fn parse_filtered_accessor() {
        check(
            "#z_mixins: @local::zalgo~>filter(test, more_test, 4+4)",
            expect![[r##"
Root@0..54
  Accessor@0..54
    Name@0..11
      Octothorpe@0..1 "#"
      Ident@1..9 "z_mixins"
      Colon@9..10 ":"
      Whitespace@10..11 " "
    Path@11..54
      Name@11..17
        At@11..12 "@"
        Ident@12..17 "local"
      ColonColon@17..19 "::"
      Path@19..54
        Name@19..24
          Ident@19..24 "zalgo"
        FuncCall@24..54
          RSquiggleArrow@24..26 "~>"
          Path@26..54
            Name@26..32
              Ident@26..32 "filter"
            FuncArgs@32..54
              LParen@32..33 "("
              Ref@33..37
                Path@33..37
                  Name@33..37
                    Ident@33..37 "test"
              Comma@37..38 ","
              Whitespace@38..39 " "
              Ref@39..48
                Path@39..48
                  Name@39..48
                    Ident@39..48 "more_test"
              Comma@48..49 ","
              Whitespace@49..50 " "
              InfixExpr@50..53
                Literal@50..51
                  Number@50..51 "4"
                Plus@51..52 "+"
                Literal@52..53
                  Number@52..53 "4"
              RParen@53..54 ")""##]],
        )
    }
}
