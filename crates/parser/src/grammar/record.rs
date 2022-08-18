use super::*;

pub(super) fn build_standard(p: &mut Parser) -> Option<CompletedMarker> {
    let mut m = p.start();
    p.expect(TokenKind::Ident);
    p.if_bump(TokenKind::QMark); // for nullable records.
    p.expect(TokenKind::Colon);
    m = m.complete(p, SyntaxKind::Name).precede(p);

    // if p.at(TokenKind::LBird) {
    //     scope_stmt::expand(p);
    // }

    if let Some(e) = expr::expr(p) {
        if p.at(TokenKind::LBrace) {
            e.precede(p).complete(p, SyntaxKind::ConstraintExpr);
            let body = p.start();
            strct::strct(p);
            body.complete(p, SyntaxKind::Body);
        } else {
            e.precede(p).complete(p, SyntaxKind::Body);
        }
    } else {
        if p.at(TokenKind::LBrace) {
            let body = p.start();
            println!("boop");
            strct::strct(p);
            body.complete(p, SyntaxKind::Body);
        }
    }
    return Some(m.complete(p, SyntaxKind::Record));
}

pub(super) fn build_schema(p: &mut Parser) -> Option<CompletedMarker> {
    let mut m = p.start();
    p.expect(TokenKind::DollarSign);
    p.expect(TokenKind::Ident);
    p.expect(TokenKind::Colon);
    m = m.complete(p, SyntaxKind::Name).precede(p);

    if p.at(TokenKind::LBird) {
        scope_stmt::expand(p);
    }

    if let Some(e) = expr::expr(p) {
        println!("2A");
        if p.at(TokenKind::LBrace) {
            e.precede(p).complete(p, SyntaxKind::ConstraintExpr);
            let body = p.start();
            strct::strct(p);
            body.complete(p, SyntaxKind::Body);
        } else {
            e.precede(p).complete(p, SyntaxKind::Body);
        }
    } else {
        println!("2B");
        if p.at(TokenKind::LBrace) {
            let body = p.start();
            println!("boop");
            strct::strct(p);
            body.complete(p, SyntaxKind::Body);
        }
    }
    return Some(m.complete(p, SyntaxKind::Schema));
}

pub(super) fn build_anonymous(p: &mut Parser) -> Option<CompletedMarker> {
    let mut m = p.start();
    p.expect(TokenKind::RAngleBrack);
    if p.at(TokenKind::LBird) {
        scope_stmt::expand(p);
    }
    expr::expr(p);
    m = m.complete(p, SyntaxKind::Body).precede(p);
    return Some(m.complete(p, SyntaxKind::AnonymousRecord));
}

pub(super) fn build_inlined(p: &mut Parser) -> Option<CompletedMarker> {
    let m = p.start();
    p.expect(TokenKind::LSquiggleArrow);
    if p.at(TokenKind::LBird) {
        scope_stmt::expand(p);
    }
    ident::expand(p);

    return Some(m.complete(p, SyntaxKind::InlinedRecord));
}

#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_accessor_dec() {
        check(
            r#"#zalgo: @global::zalgo"#,
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
    fn parse_record_with_unbracketed_sections() {
        check(
            r#" aStruct: {
    --- #primaryFields: 
    A: 1 * 0
    --- #secondaryFields:
    prim: A..D
    --- #groups:
    AB: A & B
}"#,
            expect![[r##"
Root@0..123
  Whitespace@0..1 " "
  Record@1..123
    Name@1..10
      Ident@1..8 "aStruct"
      Colon@8..9 ":"
      Whitespace@9..10 " "
    Body@10..123
      Struct@10..123
        LBrace@10..11 "{"
        Whitespace@11..16 "\n    "
        Section@16..54
          SectionMarker@16..19 "---"
          Whitespace@19..20 " "
          Name@20..41
            Path@20..34
              Name@20..34
                Octothorpe@20..21 "#"
                Ident@21..34 "primaryFields"
            Colon@34..35 ":"
            Whitespace@35..41 " \n    "
          Record@41..54
            Name@41..44
              Ident@41..42 "A"
              Colon@42..43 ":"
              Whitespace@43..44 " "
            Body@44..54
              InfixExpr@44..54
                Literal@44..46
                  Number@44..45 "1"
                  Whitespace@45..46 " "
                Star@46..47 "*"
                Whitespace@47..48 " "
                Literal@48..54
                  Number@48..49 "0"
                  Whitespace@49..54 "\n    "
        Section@54..95
          SectionMarker@54..57 "---"
          Whitespace@57..58 " "
          Name@58..80
            Path@58..74
              Name@58..74
                Octothorpe@58..59 "#"
                Ident@59..74 "secondaryFields"
            Colon@74..75 ":"
            Whitespace@75..80 "\n    "
          Record@80..95
            Name@80..86
              Ident@80..84 "prim"
              Colon@84..85 ":"
              Whitespace@85..86 " "
            Body@86..95
              InfixExpr@86..95
                Ref@86..87
                  Path@86..87
                    Name@86..87
                      Ident@86..87 "A"
                DotDot@87..89 ".."
                Ref@89..95
                  Path@89..95
                    Name@89..95
                      Ident@89..90 "D"
                      Whitespace@90..95 "\n    "
        Section@95..122
          SectionMarker@95..98 "---"
          Whitespace@98..99 " "
          Name@99..112
            Path@99..106
              Name@99..106
                Octothorpe@99..100 "#"
                Ident@100..106 "groups"
            Colon@106..107 ":"
            Whitespace@107..112 "\n    "
          Record@112..122
            Name@112..116
              Ident@112..114 "AB"
              Colon@114..115 ":"
              Whitespace@115..116 " "
            Body@116..122
              InfixExpr@116..122
                Ref@116..118
                  Path@116..118
                    Name@116..118
                      Ident@116..117 "A"
                      Whitespace@117..118 " "
                Ampersand@118..119 "&"
                Whitespace@119..120 " "
                Ref@120..122
                  Path@120..122
                    Name@120..122
                      Ident@120..121 "B"
                      Whitespace@121..122 "\n"
        RBrace@122..123 "}""##]],
        )
    }

    #[test]
    fn parse_record_with_bracketed_sections() {
        check(
            r#" aStruct: {
    --- #primaryFields: {
      A: 1 * 0
    }
    --- #secondaryFields: {
      prim: A..D
    }
    --- #groups: {
      AB: A & B
    }
}"#,
            expect![[r##"
Root@0..152
  Whitespace@0..1 " "
  Record@1..152
    Name@1..10
      Ident@1..8 "aStruct"
      Colon@8..9 ":"
      Whitespace@9..10 " "
    Body@10..152
      Struct@10..152
        LBrace@10..11 "{"
        Whitespace@11..16 "\n    "
        Section@16..63
          SectionMarker@16..19 "---"
          Whitespace@19..20 " "
          Name@20..36
            Path@20..34
              Name@20..34
                Octothorpe@20..21 "#"
                Ident@21..34 "primaryFields"
            Colon@34..35 ":"
            Whitespace@35..36 " "
          LBrace@36..37 "{"
          Whitespace@37..44 "\n      "
          Record@44..57
            Name@44..47
              Ident@44..45 "A"
              Colon@45..46 ":"
              Whitespace@46..47 " "
            Body@47..57
              InfixExpr@47..57
                Literal@47..49
                  Number@47..48 "1"
                  Whitespace@48..49 " "
                Star@49..50 "*"
                Whitespace@50..51 " "
                Literal@51..57
                  Number@51..52 "0"
                  Whitespace@52..57 "\n    "
          RBrace@57..58 "}"
          Whitespace@58..63 "\n    "
        Section@63..114
          SectionMarker@63..66 "---"
          Whitespace@66..67 " "
          Name@67..85
            Path@67..83
              Name@67..83
                Octothorpe@67..68 "#"
                Ident@68..83 "secondaryFields"
            Colon@83..84 ":"
            Whitespace@84..85 " "
          LBrace@85..86 "{"
          Whitespace@86..93 "\n      "
          Record@93..108
            Name@93..99
              Ident@93..97 "prim"
              Colon@97..98 ":"
              Whitespace@98..99 " "
            Body@99..108
              InfixExpr@99..108
                Ref@99..100
                  Path@99..100
                    Name@99..100
                      Ident@99..100 "A"
                DotDot@100..102 ".."
                Ref@102..108
                  Path@102..108
                    Name@102..108
                      Ident@102..103 "D"
                      Whitespace@103..108 "\n    "
          RBrace@108..109 "}"
          Whitespace@109..114 "\n    "
        Section@114..151
          SectionMarker@114..117 "---"
          Whitespace@117..118 " "
          Name@118..127
            Path@118..125
              Name@118..125
                Octothorpe@118..119 "#"
                Ident@119..125 "groups"
            Colon@125..126 ":"
            Whitespace@126..127 " "
          LBrace@127..128 "{"
          Whitespace@128..135 "\n      "
          Record@135..149
            Name@135..139
              Ident@135..137 "AB"
              Colon@137..138 ":"
              Whitespace@138..139 " "
            Body@139..149
              InfixExpr@139..149
                Ref@139..141
                  Path@139..141
                    Name@139..141
                      Ident@139..140 "A"
                      Whitespace@140..141 " "
                Ampersand@141..142 "&"
                Whitespace@142..143 " "
                Ref@143..149
                  Path@143..149
                    Name@143..149
                      Ident@143..144 "B"
                      Whitespace@144..149 "\n    "
          RBrace@149..150 "}"
          Whitespace@150..151 "\n"
        RBrace@151..152 "}""##]],
        )
    }

    #[test]
    fn parse_record_with_namespaced_anon_section() {
        check(
            r#" aStruct: {
    --- <; @global::zalgo::mixins ;>
      A: 1 * skippy
}"#,
            expect![[r##"
Root@0..70
  Whitespace@0..1 " "
  Record@1..70
    Name@1..10
      Ident@1..8 "aStruct"
      Colon@8..9 ":"
      Whitespace@9..10 " "
    Body@10..70
      Struct@10..70
        LBrace@10..11 "{"
        Whitespace@11..16 "\n    "
        Section@16..69
          SectionMarker@16..19 "---"
          Whitespace@19..20 " "
          ScopeBlock@20..55
            List@20..55
              LBird@20..22 "<;"
              Whitespace@22..23 " "
              Ref@23..46
                Path@23..46
                  Name@23..30
                    At@23..24 "@"
                    Ident@24..30 "global"
                  ColonColon@30..32 "::"
                  Path@32..46
                    Name@32..37
                      Ident@32..37 "zalgo"
                    ColonColon@37..39 "::"
                    Path@39..46
                      Name@39..46
                        Ident@39..45 "mixins"
                        Whitespace@45..46 " "
              RBird@46..48 ";>"
              Whitespace@48..55 "\n      "
          Record@55..69
            Name@55..58
              Ident@55..56 "A"
              Colon@56..57 ":"
              Whitespace@57..58 " "
            Body@58..69
              InfixExpr@58..69
                Literal@58..60
                  Number@58..59 "1"
                  Whitespace@59..60 " "
                Star@60..61 "*"
                Whitespace@61..62 " "
                Ref@62..69
                  Path@62..69
                    Name@62..69
                      Ident@62..68 "skippy"
                      Whitespace@68..69 "\n"
        RBrace@69..70 "}""##]],
        )
    }

    #[test]
    fn parse_record_with_nested_sections() {
        check(
r#" 
aStruct: {
    --- #outer: {
        --- #inner_bracketed: {
            A: 1
        }
        B: 2
        --- #inner_no_brackets:
        C: 2
    }
    D: 3
}"#,
            expect![[r##"
Root@0..164
  Whitespace@0..2 " \n"
  Record@2..164
    Name@2..11
      Ident@2..9 "aStruct"
      Colon@9..10 ":"
      Whitespace@10..11 " "
    Body@11..164
      Struct@11..164
        LBrace@11..12 "{"
        Whitespace@12..17 "\n    "
        Section@17..158
          SectionMarker@17..20 "---"
          Whitespace@20..21 " "
          Name@21..29
            Path@21..27
              Name@21..27
                Octothorpe@21..22 "#"
                Ident@22..27 "outer"
            Colon@27..28 ":"
            Whitespace@28..29 " "
          LBrace@29..30 "{"
          Whitespace@30..39 "\n        "
          Section@39..98
            SectionMarker@39..42 "---"
            Whitespace@42..43 " "
            Name@43..61
              Path@43..59
                Name@43..59
                  Octothorpe@43..44 "#"
                  Ident@44..59 "inner_bracketed"
              Colon@59..60 ":"
              Whitespace@60..61 " "
            LBrace@61..62 "{"
            Whitespace@62..75 "\n            "
            Record@75..88
              Name@75..78
                Ident@75..76 "A"
                Colon@76..77 ":"
                Whitespace@77..78 " "
              Body@78..88
                Literal@78..88
                  Number@78..79 "1"
                  Whitespace@79..88 "\n        "
            RBrace@88..89 "}"
            Whitespace@89..98 "\n        "
          Record@98..111
            Name@98..101
              Ident@98..99 "B"
              Colon@99..100 ":"
              Whitespace@100..101 " "
            Body@101..111
              Literal@101..111
                Number@101..102 "2"
                Whitespace@102..111 "\n        "
          Section@111..152
            SectionMarker@111..114 "---"
            Whitespace@114..115 " "
            Name@115..143
              Path@115..133
                Name@115..133
                  Octothorpe@115..116 "#"
                  Ident@116..133 "inner_no_brackets"
              Colon@133..134 ":"
              Whitespace@134..143 "\n        "
            Record@143..152
              Name@143..146
                Ident@143..144 "C"
                Colon@144..145 ":"
                Whitespace@145..146 " "
              Body@146..152
                Literal@146..152
                  Number@146..147 "2"
                  Whitespace@147..152 "\n    "
          RBrace@152..153 "}"
          Whitespace@153..158 "\n    "
        Record@158..163
          Name@158..161
            Ident@158..159 "D"
            Colon@159..160 ":"
            Whitespace@160..161 " "
          Body@161..163
            Literal@161..163
              Number@161..162 "3"
              Whitespace@162..163 "\n"
        RBrace@163..164 "}""##]],
        )
    }

    // Should the namespace statement be BEFORE the colon?
    #[test]
    fn parse_namespaced_constrained_record() {
        check(
r#" 
aStruct: <; @global::mixins::ctypes ;> & $someSchema & $anotherSchema & {
    D: 3
}"#,
            expect![[r##"
Root@0..86
  Whitespace@0..2 " \n"
  Record@2..86
    Name@2..11
      Ident@2..9 "aStruct"
      Colon@9..10 ":"
      Whitespace@10..11 " "
    Body@11..86
      InfixExpr@11..86
        InfixExpr@11..72
          InfixExpr@11..55
            ScopeBlock@11..41
              List@11..41
                LBird@11..13 "<;"
                Whitespace@13..14 " "
                Ref@14..38
                  Path@14..38
                    Name@14..21
                      At@14..15 "@"
                      Ident@15..21 "global"
                    ColonColon@21..23 "::"
                    Path@23..38
                      Name@23..29
                        Ident@23..29 "mixins"
                      ColonColon@29..31 "::"
                      Path@31..38
                        Name@31..38
                          Ident@31..37 "ctypes"
                          Whitespace@37..38 " "
                RBird@38..40 ";>"
                Whitespace@40..41 " "
            Ampersand@41..42 "&"
            Whitespace@42..43 " "
            Type@43..55
              Path@43..55
                Name@43..55
                  DollarSign@43..44 "$"
                  Ident@44..54 "someSchema"
                  Whitespace@54..55 " "
          Ampersand@55..56 "&"
          Whitespace@56..57 " "
          Type@57..72
            Path@57..72
              Name@57..72
                DollarSign@57..58 "$"
                Ident@58..71 "anotherSchema"
                Whitespace@71..72 " "
        Ampersand@72..73 "&"
        Whitespace@73..74 " "
        Struct@74..86
          LBrace@74..75 "{"
          Whitespace@75..80 "\n    "
          Record@80..85
            Name@80..83
              Ident@80..81 "D"
              Colon@81..82 ":"
              Whitespace@82..83 " "
            Body@83..85
              Literal@83..85
                Number@83..84 "3"
                Whitespace@84..85 "\n"
          RBrace@85..86 "}""##]],
        )
    }


}
