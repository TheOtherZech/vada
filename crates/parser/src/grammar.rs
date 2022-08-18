mod expr;
mod stmt;
mod record;
mod accessor;
mod func;
mod directive;
mod strct;
mod ident;
mod scope_stmt;
mod list;
mod expand;

use crate::parser::marker::CompletedMarker;
use crate::parser::Parser;
use lexer::TokenKind;
use syntax::SyntaxKind;

pub(crate) fn root(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    while !p.at_end() {
        stmt::stmt(p);
    }

    m.complete(p, SyntaxKind::Root)
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_monorecord() {
      check(
"some: 1",
expect![[
r#"
Root@0..7
  Record@0..7
    Name@0..6
      Ident@0..4 "some"
      Colon@4..5 ":"
      Whitespace@5..6 " "
    Body@6..7
      Literal@6..7
        Number@6..7 "1""#]])

    }

    #[test]
    fn parse_multiple_statements() {
        check(
            "a : 1\nb : a",
            expect![[r#"
Root@0..11
  Record@0..6
    Name@0..4
      Ident@0..1 "a"
      Whitespace@1..2 " "
      Colon@2..3 ":"
      Whitespace@3..4 " "
    Body@4..6
      Literal@4..6
        Number@4..5 "1"
        Whitespace@5..6 "\n"
  Record@6..11
    Name@6..10
      Ident@6..7 "b"
      Whitespace@7..8 " "
      Colon@8..9 ":"
      Whitespace@9..10 " "
    Body@10..11
      Ref@10..11
        Path@10..11
          Name@10..11
            Ident@10..11 "a""#]],
        );
    }

    #[test]
    fn parse_nested_structs() {
        check(
r#"outerRecord:{name:"jimbo"innerRecord:{age:12}}"#,
            expect![[r#"
Root@0..46
  Record@0..46
    Name@0..12
      Ident@0..11 "outerRecord"
      Colon@11..12 ":"
    Body@12..46
      Struct@12..46
        LBrace@12..13 "{"
        Record@13..25
          Name@13..18
            Ident@13..17 "name"
            Colon@17..18 ":"
          Body@18..25
            Literal@18..25
              String@18..25 "\"jimbo\""
        Record@25..45
          Name@25..37
            Ident@25..36 "innerRecord"
            Colon@36..37 ":"
          Body@37..45
            Struct@37..45
              LBrace@37..38 "{"
              Record@38..44
                Name@38..42
                  Ident@38..41 "age"
                  Colon@41..42 ":"
                Body@42..44
                  Literal@42..44
                    Number@42..44 "12"
              RBrace@44..45 "}"
        RBrace@45..46 "}""#]],
        );
    }

    // TODO: Update this for new inline syntax
    #[test]
    fn parse_inlined_record() {
        check(
r#"outerRecord:{name:"jimbo"<~innerRecord}"#,
            expect![[r#"
Root@0..39
  Record@0..39
    Name@0..12
      Ident@0..11 "outerRecord"
      Colon@11..12 ":"
    Body@12..39
      Struct@12..39
        LBrace@12..13 "{"
        Record@13..25
          Name@13..18
            Ident@13..17 "name"
            Colon@17..18 ":"
          Body@18..25
            Literal@18..25
              String@18..25 "\"jimbo\""
        InlinedRecord@25..38
          LSquiggleArrow@25..27 "<~"
          Path@27..38
            Name@27..38
              Ident@27..38 "innerRecord"
        RBrace@38..39 "}""#]],
        );
    }


    #[test]
    fn parse_constrained_record() {
      check(
r#"someRecord: aSchema & anotherSchema {
  name: "Jimmy"
  age: 45
}"#,
expect![[r#"
Root@0..65
  Record@0..65
    Name@0..12
      Ident@0..10 "someRecord"
      Colon@10..11 ":"
      Whitespace@11..12 " "
    ConstraintExpr@12..36
      InfixExpr@12..36
        Ref@12..20
          Path@12..20
            Name@12..20
              Ident@12..19 "aSchema"
              Whitespace@19..20 " "
        Ampersand@20..21 "&"
        Whitespace@21..22 " "
        Ref@22..36
          Path@22..36
            Name@22..36
              Ident@22..35 "anotherSchema"
              Whitespace@35..36 " "
    Body@36..65
      Struct@36..65
        LBrace@36..37 "{"
        Whitespace@37..40 "\n  "
        Record@40..56
          Name@40..46
            Ident@40..44 "name"
            Colon@44..45 ":"
            Whitespace@45..46 " "
          Body@46..56
            Literal@46..56
              String@46..53 "\"Jimmy\""
              Whitespace@53..56 "\n  "
        Record@56..64
          Name@56..61
            Ident@56..59 "age"
            Colon@59..60 ":"
            Whitespace@60..61 " "
          Body@61..64
            Literal@61..64
              Number@61..63 "45"
              Whitespace@63..64 "\n"
        RBrace@64..65 "}""#]])
    }

    #[test]
    fn parse_schema(){
        check(
r#"$personSchema: {
    name: $string
    desc: $string
    tags: [...$string]
    test: 2+2
    anotherTest: 1*2+3*4
    pickle: 1+2&3+4
    age: $int & >= 0
    somelist: [$string, $string, $string]
}"#,
            expect![[r##"
Root@0..199
  Schema@0..199
    Name@0..15
      DollarSign@0..1 "$"
      Ident@1..13 "personSchema"
      Colon@13..14 ":"
      Whitespace@14..15 " "
    Body@15..199
      Struct@15..199
        LBrace@15..16 "{"
        Whitespace@16..21 "\n    "
        Record@21..39
          Name@21..27
            Ident@21..25 "name"
            Colon@25..26 ":"
            Whitespace@26..27 " "
          Body@27..39
            Type@27..39
              Path@27..39
                Name@27..39
                  DollarSign@27..28 "$"
                  Ident@28..34 "string"
                  Whitespace@34..39 "\n    "
        Record@39..57
          Name@39..45
            Ident@39..43 "desc"
            Colon@43..44 ":"
            Whitespace@44..45 " "
          Body@45..57
            Type@45..57
              Path@45..57
                Name@45..57
                  DollarSign@45..46 "$"
                  Ident@46..52 "string"
                  Whitespace@52..57 "\n    "
        Record@57..80
          Name@57..63
            Ident@57..61 "tags"
            Colon@61..62 ":"
            Whitespace@62..63 " "
          Body@63..80
            List@63..80
              LBrack@63..64 "["
              PrefixExpr@64..74
                DotDotDot@64..67 "..."
                Type@67..74
                  Path@67..74
                    Name@67..74
                      DollarSign@67..68 "$"
                      Ident@68..74 "string"
              RBrack@74..75 "]"
              Whitespace@75..80 "\n    "
        Record@80..94
          Name@80..86
            Ident@80..84 "test"
            Colon@84..85 ":"
            Whitespace@85..86 " "
          Body@86..94
            InfixExpr@86..94
              Literal@86..87
                Number@86..87 "2"
              Plus@87..88 "+"
              Literal@88..94
                Number@88..89 "2"
                Whitespace@89..94 "\n    "
        Record@94..119
          Name@94..107
            Ident@94..105 "anotherTest"
            Colon@105..106 ":"
            Whitespace@106..107 " "
          Body@107..119
            InfixExpr@107..119
              InfixExpr@107..110
                Literal@107..108
                  Number@107..108 "1"
                Star@108..109 "*"
                Literal@109..110
                  Number@109..110 "2"
              Plus@110..111 "+"
              InfixExpr@111..119
                Literal@111..112
                  Number@111..112 "3"
                Star@112..113 "*"
                Literal@113..119
                  Number@113..114 "4"
                  Whitespace@114..119 "\n    "
        Record@119..139
          Name@119..127
            Ident@119..125 "pickle"
            Colon@125..126 ":"
            Whitespace@126..127 " "
          Body@127..139
            InfixExpr@127..139
              InfixExpr@127..130
                Literal@127..128
                  Number@127..128 "1"
                Plus@128..129 "+"
                Literal@129..130
                  Number@129..130 "2"
              Ampersand@130..131 "&"
              InfixExpr@131..139
                Literal@131..132
                  Number@131..132 "3"
                Plus@132..133 "+"
                Literal@133..139
                  Number@133..134 "4"
                  Whitespace@134..139 "\n    "
        Record@139..160
          Name@139..144
            Ident@139..142 "age"
            Colon@142..143 ":"
            Whitespace@143..144 " "
          Body@144..160
            InfixExpr@144..160
              Type@144..149
                Path@144..149
                  Name@144..149
                    DollarSign@144..145 "$"
                    Ident@145..148 "int"
                    Whitespace@148..149 " "
              Ampersand@149..150 "&"
              Whitespace@150..151 " "
              PrefixExpr@151..160
                GreaterEqual@151..153 ">="
                Whitespace@153..154 " "
                Literal@154..160
                  Number@154..155 "0"
                  Whitespace@155..160 "\n    "
        Record@160..198
          Name@160..170
            Ident@160..168 "somelist"
            Colon@168..169 ":"
            Whitespace@169..170 " "
          Body@170..198
            List@170..198
              LBrack@170..171 "["
              Type@171..178
                Path@171..178
                  Name@171..178
                    DollarSign@171..172 "$"
                    Ident@172..178 "string"
              Comma@178..179 ","
              Whitespace@179..180 " "
              Type@180..187
                Path@180..187
                  Name@180..187
                    DollarSign@180..181 "$"
                    Ident@181..187 "string"
              Comma@187..188 ","
              Whitespace@188..189 " "
              Type@189..196
                Path@189..196
                  Name@189..196
                    DollarSign@189..190 "$"
                    Ident@190..196 "string"
              RBrack@196..197 "]"
              Whitespace@197..198 "\n"
        RBrace@198..199 "}""##]],
        )
    }

    #[test]
    fn parse_record(){
        check(
r#"person: {
    name: "Jimbo"
    desc: "A Man of Many Talents"
    tags: ["Dude", "Bro", "Broskito"]
    age: 34
}"#,
            expect![[r##"
Root@0..113
  Record@0..113
    Name@0..8
      Ident@0..6 "person"
      Colon@6..7 ":"
      Whitespace@7..8 " "
    Body@8..113
      Struct@8..113
        LBrace@8..9 "{"
        Whitespace@9..14 "\n    "
        Record@14..32
          Name@14..20
            Ident@14..18 "name"
            Colon@18..19 ":"
            Whitespace@19..20 " "
          Body@20..32
            Literal@20..32
              String@20..27 "\"Jimbo\""
              Whitespace@27..32 "\n    "
        Record@32..66
          Name@32..38
            Ident@32..36 "desc"
            Colon@36..37 ":"
            Whitespace@37..38 " "
          Body@38..66
            Literal@38..66
              String@38..61 "\"A Man of Many Talents\""
              Whitespace@61..66 "\n    "
        Record@66..104
          Name@66..72
            Ident@66..70 "tags"
            Colon@70..71 ":"
            Whitespace@71..72 " "
          Body@72..104
            List@72..104
              LBrack@72..73 "["
              Literal@73..79
                String@73..79 "\"Dude\""
              Comma@79..80 ","
              Whitespace@80..81 " "
              Literal@81..86
                String@81..86 "\"Bro\""
              Comma@86..87 ","
              Whitespace@87..88 " "
              Literal@88..98
                String@88..98 "\"Broskito\""
              RBrack@98..99 "]"
              Whitespace@99..104 "\n    "
        Record@104..112
          Name@104..109
            Ident@104..107 "age"
            Colon@107..108 ":"
            Whitespace@108..109 " "
          Body@109..112
            Literal@109..112
              Number@109..111 "34"
              Whitespace@111..112 "\n"
        RBrace@112..113 "}""##]],
        )
    }
}