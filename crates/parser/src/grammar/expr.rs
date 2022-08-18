use super::*;
use crate::parser::marker::Marker;

const PREFIX_TOKENS: [TokenKind; 13] = [
    // Yes, all of the basic math symbols can be prefixes.
    //
    TokenKind::Plus,    // explicit-positive
    TokenKind::Minus,   // explicit-negative
    TokenKind::Star,    // multiple-of constraint
    TokenKind::Slash,   // divisor-of constraint
    TokenKind::Carrot,  // power-of constraint
    TokenKind::Percent, // ...?
    TokenKind::Colon,   // Bind alias
    // TokenKind::DollarSign,
    TokenKind::LessEqual,
    TokenKind::GreaterEqual,
    TokenKind::LAngleBrack,
    TokenKind::RAngleBrack,
    TokenKind::DotDot,
    TokenKind::DotDotDot,
];

const LITERAL_TOKENS: [TokenKind; 9] = [
    TokenKind::HexLiteral,
    TokenKind::IntLiteral,
    TokenKind::FloatLiteral,
    TokenKind::OctalLiteral,
    TokenKind::BinaryLiteral,
    TokenKind::AddressLiteral,
    TokenKind::RandIntLiteral,
    TokenKind::RandFloatLiteral,
    TokenKind::String,
];

const POSTFIX_TOKENS: [TokenKind; 1] = [TokenKind::DotDot];

pub(super) fn expr(p: &mut Parser) -> Option<CompletedMarker> {
    expr_binding_power(p, 0, true)
}

fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8, do_list: bool) -> Option<CompletedMarker> {
    let mut lhs = lhs(p, do_list)?;
    loop {
        let op = infix_expr(p);
        if op == BinaryOp::Err {
            break;
        }

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            break;
        }

        p.bump();

        let m = lhs.precede(p);
        let parsed_rhs = expr_binding_power(p, right_binding_power, do_list).is_some();
        lhs = m.complete(p, SyntaxKind::InfixExpr);

        if !parsed_rhs {
            break;
        }
    }
    Some(lhs)
}

fn at_prefix(p: &mut Parser) -> bool {
    for t in PREFIX_TOKENS.into_iter() {
        if p.at(t) {
            return true;
        }
    }
    return false;
}

fn at_postfix(p: &mut Parser) -> bool {
    for t in POSTFIX_TOKENS.into_iter() {
        if p.was_at(t) {
            return true;
        }
    }
    return false;
}

fn at_literal(p: &mut Parser) -> bool {
    for t in LITERAL_TOKENS.into_iter() {
        if p.at(t) {
            return true;
        }
    }
    return false;
}

fn lhs(p: &mut Parser, do_list: bool) -> Option<CompletedMarker> {
    let cm = 
    if p.at_expandable() {
      let t = expand::expand(p, false).unwrap();
      match t.extract(p) {
        SyntaxKind::Name      => { t.precede(p).complete(p, SyntaxKind::Ref) },
        SyntaxKind::Transform => { 
          let m = t.precede(p);
          transform_expr(p);
          m.complete(p, SyntaxKind::PrefixExpr)
         },
        _                     => { t }
      }
    }
    else if at_literal(p) {
        literal(p)
    }
    else if p.at(TokenKind::LBrace) {
        strct::strct(p)
    }
    else if at_prefix(p) {
        prefix_expr(p)
    } 
    else if p.at(TokenKind::LParen) {
        paren_expr(p)
    }
    else if p.at(TokenKind::LBrack) {
      build_list(p, None)
    }
    else if p.at(TokenKind::LBird) {
        scope_stmt::expand(p).unwrap() // Change to bind syntax?
    }
    else {
        if at_postfix(p) {
        }
        else if p.at(TokenKind::LBrace) {
            println!("Brace found, exiting expression");
        } 
        else {
            println!("LHS Err");
            p.error();
        }
        return None;
    };



    if p.at(TokenKind::Comma) || p.at(TokenKind::Semicolon) {
      if do_list {
        return Some(build_list(p, Some(cm)));
      }
    }
    else {
      return Some(cm);
    }

    if do_list {

    }

    Some(cm)
}

#[derive(PartialEq)]
enum BinaryOp {
    Add,     // +
    Sub,     // -
    Mul,     // *
    Div,     // /
    Mod,     // %
    Unify,   // &
    Dsj,     // |
    Range,   // ..
    Err,

    // Pipes
    DirArrow, // ->
    Seq,      // ~>

    // Accessors
    ScopeRes,  // ::
    MemberRes, // .
    
}

impl BinaryOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            // Always the loosest-binding Operator
            Self::Seq                         => (0, 0),
            Self::Dsj                         => (1, 0), // Note left-associative
            Self::Unify                       => (2, 3), 
            Self::Add | Self::Sub             => (4, 5),
            Self::Mul | Self::Div 
                      | Self::Mod             => (6, 7),
            
            Self::DirArrow                    => (8, 9),

            // Resolvers are always the tightest-binding operators
            Self::ScopeRes | Self::MemberRes  => (10, 11),
            
            
            Self::Range => (120, 120),
            _ => (0, 0),
        }
    }
}
#[derive(PartialEq)]
enum UnaryOp {
    Pos,       // + Plus
    Neg,       // - Minus
    Mul,       // * Star
    Div,       // / Slash
    Exp,       // ^ Carrot
    Mod,       // % Percent
    // Comparators for constraints
    Gr,    // >
    GrEq,  // >=
    Ls,    // <
    LsEq,  // <=
    Range, // ..
    Open,  // ...


    // Experimental Stuff Here
    // Single arity functions are
    Func,      // #Transform, etc
    Err,
}

impl UnaryOp {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Func              => (0, 0), // Unary zero-bind for single-arity transforms

            Self::Gr | Self::GrEq 
            | Self::Ls | Self::LsEq => (0, 5),

            Self::Pos | Self::Neg 
            | Self::Mul | Self::Div 
            | Self::Exp | Self::Mod => (0, 11),

            Self::Open | Self::Range => (0, 13),
            _ => (0, 0),
        }
    }
}

fn literal(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::Literal)
}

fn infix_expr(p: &mut Parser) -> BinaryOp {
    let op = if p.at(TokenKind::Plus) {
        BinaryOp::Add
    } else if p.at(TokenKind::Minus) {
        BinaryOp::Sub
    } else if p.at(TokenKind::Star) {
        BinaryOp::Mul
    } else if p.at(TokenKind::Slash) {
        BinaryOp::Div
    } else if p.at(TokenKind::Percent) {
        BinaryOp::Mod
    } else if p.at(TokenKind::DotDot) {
        // This is... Horrible.
        if p.at_next(TokenKind::Equals) {
            p.bump();
        }
        BinaryOp::Range
    } else if p.at(TokenKind::Ampersand) {
        BinaryOp::Unify
    } else if p.at(TokenKind::Bar) {
        BinaryOp::Dsj
    } else if p.at(TokenKind::RSquiggleArrow) {
        BinaryOp::Seq
    } else if p.at(TokenKind::RArrow) {
        BinaryOp::DirArrow
    } else if p.at(TokenKind::Dot) {
        BinaryOp::MemberRes
    } else if p.at(TokenKind::ColonColon) {
        BinaryOp::ScopeRes
    } else {
        BinaryOp::Err 
    };

    return op;
}

fn transform_expr(p: &mut Parser) {
  println!("Look ma!");


  // Low binding open expression
  if p.at(TokenKind::Colon) {
    p.bump();
    expr_binding_power(p, 1, false);
  }
  // High binding closed expression
  else {
    expr_binding_power(p, 10, false);
  }


}

/// Handles both math prefixes and record prefixes
fn prefix_expr(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    let op = if p.at(TokenKind::Plus) {
        UnaryOp::Pos
    } else if p.at(TokenKind::Minus) {
        UnaryOp::Neg
    } else if p.at(TokenKind::Star) {
        UnaryOp::Mul
    } else if p.at(TokenKind::Slash) {
        UnaryOp::Div
    } else if p.at(TokenKind::Carrot) {
        UnaryOp::Exp
    } else if p.at(TokenKind::Percent) {
        UnaryOp::Mod
    } else if p.at(TokenKind::LAngleBrack) {
        UnaryOp::Ls
    } else if p.at(TokenKind::RAngleBrack) {
        UnaryOp::Gr
    } else if p.at(TokenKind::GreaterEqual) {
        UnaryOp::GrEq
    } else if p.at(TokenKind::LessEqual) {
        UnaryOp::LsEq
    } else if p.at(TokenKind::DotDotDot) {
        UnaryOp::Open
    } else if p.at(TokenKind::DotDot) {
        UnaryOp::Range
    } else if p.at(TokenKind::Octothorpe) {
        UnaryOp::Func
    } else {
        p.error();
        UnaryOp::Err
    };

    let (left_binding_power, right_binding_power) = op.binding_power();

    if left_binding_power > 0 {}
    // Eat the operators token.
    if op == UnaryOp::Func {
      expand::expand(p, false);
      p.expect(TokenKind::Colon);
    } 
    else {
      p.bump();
    }

    expr_binding_power(p, right_binding_power, false);

    m.complete(p, SyntaxKind::PrefixExpr)
}

fn paren_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(TokenKind::LParen));

    let m = p.start();
    p.bump();
    expr_binding_power(p, 0, false);
    p.expect(TokenKind::RParen);

    m.complete(p, SyntaxKind::ParenExpr)
}

fn build_list(p: &mut Parser, start: Option<CompletedMarker>) -> CompletedMarker {
  println!("top of build list");
  let mut outer: Option<Marker> = None;
  let mut close: bool = false;
  let mut row: Marker = if start.is_some() {
    start.unwrap().precede(p).complete(p, SyntaxKind::Entry).precede(p)
  } else {
    close = p.if_bump(TokenKind::LBrack);
    p.start()
  };

  loop {
      if p.at(TokenKind::Comma) {
          p.bump();
      }

      if p.at(TokenKind::Semicolon) {
          println!("starting list column");
          p.bump();
          if outer.is_none() {
            outer = Some(row.complete(p, SyntaxKind::Row).precede(p));
          }
          else {
            row.complete(p, SyntaxKind::Row);
          }

          row = p.start();
          // outer = p.start();
      }

      if p.at_end() {
          println!("Break list at end of input");
          break;
      }

      let exp = expr_binding_power(p, 1, false);
      if exp.is_some() {
          exp.unwrap().precede(p).complete(p, SyntaxKind::Entry);
      } else {
        println!("breaking list");
          break;
      }

      if p.at_op() {
        break;
      }
  }

  if close == true {
    p.expect(TokenKind::RBrack);
  }

  if outer.is_some() {
    row.complete(p, SyntaxKind::Row);
    return outer.unwrap().complete(p, SyntaxKind::List);
  } else {
    return row.complete(p, SyntaxKind::Row).precede(p).complete(p, SyntaxKind::List);
  }  
}


#[cfg(test)]
mod tests {
    use crate::check;
    use expect_test::expect;

    #[test]
    fn parse_number() {
        check(
            "> 123",
            expect![[r#"
Root@0..5
  AnonymousRecord@0..5
    Body@0..5
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      Literal@2..5
        Number@2..5 "123""#]],
        );
    }

    #[test]
    fn parse_number_preceded_by_whitespace() {
        check(
            ">   9876",
            expect![[r#"
Root@0..8
  AnonymousRecord@0..8
    Body@0..8
      RAngleBrack@0..1 ">"
      Whitespace@1..4 "   "
      Literal@4..8
        Number@4..8 "9876""#]],
        );
    }

    #[test]
    fn parse_number_followed_by_whitespace() {
        check(
            "> 999   ",
            expect![[r#"
Root@0..8
  AnonymousRecord@0..8
    Body@0..8
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      Literal@2..8
        Number@2..5 "999"
        Whitespace@5..8 "   ""#]],
        );
    }

    #[test]
    fn parse_number_surrounded_by_whitespace() {
        check(
            "> 123     ",
            expect![[r#"
Root@0..10
  AnonymousRecord@0..10
    Body@0..10
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      Literal@2..10
        Number@2..5 "123"
        Whitespace@5..10 "     ""#]],
        );
    }

    #[test]
    fn parse_variable_ref() {
        check(
            "> counter",
            expect![[r#"
Root@0..9
  AnonymousRecord@0..9
    Body@0..9
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      Ref@2..9
        Name@2..9
          Ident@2..9 "counter""#]],
        );
    }

    #[test]
    fn parse_keyword_math() {
        check(
            "a: @gold + @fish",
            expect![[r#"
Root@0..16
  Record@0..16
    Name@0..1
      Ident@0..1 "a"
    Colon@1..2 ":"
    Whitespace@2..3 " "
    Body@3..16
      InfixExpr@3..16
        Ref@3..9
          Keyword@3..9
            At@3..4 "@"
            Ident@4..8 "gold"
            Whitespace@8..9 " "
        Plus@9..10 "+"
        Whitespace@10..11 " "
        Ref@11..16
          Keyword@11..16
            At@11..12 "@"
            Ident@12..16 "fish""#]],
        );
    }

    #[test]
    fn parse_keyword_member_math() {
        check(
            "a: @gold.a + @fish.b",
            expect![[r#"
Root@0..20
  Record@0..20
    Name@0..1
      Ident@0..1 "a"
    Colon@1..2 ":"
    Whitespace@2..3 " "
    Body@3..20
      InfixExpr@3..20
        Ref@3..11
          Path@3..11
            Keyword@3..8
              At@3..4 "@"
              Ident@4..8 "gold"
            Dot@8..9 "."
            Name@9..11
              Ident@9..10 "a"
              Whitespace@10..11 " "
        Plus@11..12 "+"
        Whitespace@12..13 " "
        Ref@13..20
          Path@13..20
            Keyword@13..18
              At@13..14 "@"
              Ident@14..18 "fish"
            Dot@18..19 "."
            Name@19..20
              Ident@19..20 "b""#]],
        );
    }

    #[test]
    fn parse_simple_infix_expression() {
        check(
            "> 1+2",
            expect![[r#"
Root@0..5
  AnonymousRecord@0..5
    Body@0..5
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      InfixExpr@2..5
        Literal@2..3
          Number@2..3 "1"
        Plus@3..4 "+"
        Literal@4..5
          Number@4..5 "2""#]],
        );
    }

    #[test]
    fn parse_left_associative_infix_expression() {
        check(
            "> 1+2+3+4",
            expect![[r#"
Root@0..9
  AnonymousRecord@0..9
    Body@0..9
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      InfixExpr@2..9
        InfixExpr@2..7
          InfixExpr@2..5
            Literal@2..3
              Number@2..3 "1"
            Plus@3..4 "+"
            Literal@4..5
              Number@4..5 "2"
          Plus@5..6 "+"
          Literal@6..7
            Number@6..7 "3"
        Plus@7..8 "+"
        Literal@8..9
          Number@8..9 "4""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_mixed_binding_power() {
        check(
            "> 1+2*3-4",
            expect![[r#"
Root@0..9
  AnonymousRecord@0..9
    Body@0..9
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      InfixExpr@2..9
        InfixExpr@2..7
          Literal@2..3
            Number@2..3 "1"
          Plus@3..4 "+"
          InfixExpr@4..7
            Literal@4..5
              Number@4..5 "2"
            Star@5..6 "*"
            Literal@6..7
              Number@6..7 "3"
        Minus@7..8 "-"
        Literal@8..9
          Number@8..9 "4""#]],
        );
    }

    #[test]
    fn parse_infix_expression_with_whitespace() {
        check(
            "> 1 +   2* 3 ",
            expect![[r#"
Root@0..13
  AnonymousRecord@0..13
    Body@0..13
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      InfixExpr@2..13
        Literal@2..4
          Number@2..3 "1"
          Whitespace@3..4 " "
        Plus@4..5 "+"
        Whitespace@5..8 "   "
        InfixExpr@8..13
          Literal@8..9
            Number@8..9 "2"
          Star@9..10 "*"
          Whitespace@10..11 " "
          Literal@11..13
            Number@11..12 "3"
            Whitespace@12..13 " ""#]],
        );
    }

    #[test]
    fn parse_infix_expression_interspersed_with_comments() {
        check(
            r#">
1
  + 1 // Add one
  + 10 // Add ten"#,
            expect![[r##"
Root@0..38
  AnonymousRecord@0..38
    Body@0..38
      RAngleBrack@0..1 ">"
      Whitespace@1..2 "\n"
      InfixExpr@2..38
        InfixExpr@2..23
          Literal@2..6
            Number@2..3 "1"
            Whitespace@3..6 "\n  "
          Plus@6..7 "+"
          Whitespace@7..8 " "
          Literal@8..23
            Number@8..9 "1"
            Whitespace@9..10 " "
            Comment@10..20 "// Add one"
            Whitespace@20..23 "\n  "
        Plus@23..24 "+"
        Whitespace@24..25 " "
        Literal@25..38
          Number@25..27 "10"
          Whitespace@27..28 " "
          Comment@28..38 "// Add ten""##]],
        );
    }

    #[test]
    fn parse_range_expression() {
        check(
            "a: $int & 0..301",
            expect![[r#"
Root@0..16
  Record@0..16
    Name@0..1
      Ident@0..1 "a"
    Colon@1..2 ":"
    Whitespace@2..3 " "
    Body@3..16
      InfixExpr@3..16
        Ref@3..8
          Schema@3..8
            Name@3..8
              DollarSign@3..4 "$"
              Ident@4..7 "int"
              Whitespace@7..8 " "
        Ampersand@8..9 "&"
        Whitespace@9..10 " "
        InfixExpr@10..16
          Literal@10..11
            Number@10..11 "0"
          DotDot@11..13 ".."
          Literal@13..16
            Number@13..16 "301""#]],
        );
    }

    #[test]
    fn parse_inclusive_range_expression() {
        check(
            "a: $int & 0..=300",
            expect![[r#"
Root@0..17
  Record@0..17
    Name@0..1
      Ident@0..1 "a"
    Colon@1..2 ":"
    Whitespace@2..3 " "
    Body@3..17
      InfixExpr@3..17
        Ref@3..8
          Schema@3..8
            Name@3..8
              DollarSign@3..4 "$"
              Ident@4..7 "int"
              Whitespace@7..8 " "
        Ampersand@8..9 "&"
        Whitespace@9..10 " "
        InfixExpr@10..17
          Literal@10..11
            Number@10..11 "0"
          DotDot@11..13 ".."
          Equals@13..14 "="
          Literal@14..17
            Number@14..17 "300""#]],
        );
    }

    #[test]
    fn parse_prefix_implicit_range_expression() {
        check(
            "a: $int & ..300",
            expect![[r#"
Root@0..15
  Record@0..15
    Name@0..1
      Ident@0..1 "a"
    Colon@1..2 ":"
    Whitespace@2..3 " "
    Body@3..15
      InfixExpr@3..15
        Ref@3..8
          Schema@3..8
            Name@3..8
              DollarSign@3..4 "$"
              Ident@4..7 "int"
              Whitespace@7..8 " "
        Ampersand@8..9 "&"
        Whitespace@9..10 " "
        PrefixExpr@10..15
          DotDot@10..12 ".."
          Literal@12..15
            Number@12..15 "300""#]],
        );
    }

    #[test]
    fn parse_postfix_implicit_range_expression() {
        check(
            "a: $int & 300..",
            expect![[r#"
Root@0..15
  Record@0..15
    Name@0..1
      Ident@0..1 "a"
    Colon@1..2 ":"
    Whitespace@2..3 " "
    Body@3..15
      InfixExpr@3..15
        Ref@3..8
          Schema@3..8
            Name@3..8
              DollarSign@3..4 "$"
              Ident@4..7 "int"
              Whitespace@7..8 " "
        Ampersand@8..9 "&"
        Whitespace@9..10 " "
        InfixExpr@10..15
          Literal@10..13
            Number@10..13 "300"
          DotDot@13..15 "..""#]],
        );
    }

    #[test]
    fn parse_arrow() {
        check(
            "> 3 -> $uint",
            expect![[r#"
Root@0..12
  AnonymousRecord@0..12
    Body@0..12
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      InfixExpr@2..12
        Literal@2..4
          Number@2..3 "3"
          Whitespace@3..4 " "
        RArrow@4..6 "->"
        Whitespace@6..7 " "
        Ref@7..12
          Schema@7..12
            Name@7..12
              DollarSign@7..8 "$"
              Ident@8..12 "uint""#]],
        );
    }

    // TODO: Expects are... funky.
    #[test]
    fn do_not_parse_operator_if_gettting_rhs_failed() {
        check(
            "> (1+",
            expect![[r#"
Root@0..5
  AnonymousRecord@0..5
    Body@0..5
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      ParenExpr@2..5
        LParen@2..3 "("
        InfixExpr@3..5
          Literal@3..4
            Number@3..4 "1"
          Plus@4..5 "+"
error at 4..5: expected hex number, integer, float, octal number, binary number, memory address, random integer, random float, string, {, +, -, *, /, ^, %, :, <=, >=, <, >, .., ..., (, [, <; or {
error at 4..5: expected )"#]],
        );
    }

    #[test]
    fn parse_negation() {
        check(
            "> -10",
            expect![[r#"
Root@0..5
  AnonymousRecord@0..5
    Body@0..5
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      PrefixExpr@2..5
        Minus@2..3 "-"
        Literal@3..5
          Number@3..5 "10""#]],
        );
    }

    #[test]
    fn negation_has_higher_binding_power_than_binary_operators() {
        check(
            "> -20+20",
            expect![[r#"
Root@0..8
  AnonymousRecord@0..8
    Body@0..8
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      InfixExpr@2..8
        PrefixExpr@2..5
          Minus@2..3 "-"
          Literal@3..5
            Number@3..5 "20"
        Plus@5..6 "+"
        Literal@6..8
          Number@6..8 "20""#]],
        );
    }

    #[test]
    fn parse_nested_parentheses() {
        check(
            "> ((((((10))))))",
            expect![[r#"
Root@0..16
  AnonymousRecord@0..16
    Body@0..16
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      ParenExpr@2..16
        LParen@2..3 "("
        ParenExpr@3..15
          LParen@3..4 "("
          ParenExpr@4..14
            LParen@4..5 "("
            ParenExpr@5..13
              LParen@5..6 "("
              ParenExpr@6..12
                LParen@6..7 "("
                ParenExpr@7..11
                  LParen@7..8 "("
                  Literal@8..10
                    Number@8..10 "10"
                  RParen@10..11 ")"
                RParen@11..12 ")"
              RParen@12..13 ")"
            RParen@13..14 ")"
          RParen@14..15 ")"
        RParen@15..16 ")""#]],
        );
    }

    #[test]
    fn parentheses_affect_precedence() {
        check(
            "> 5*(2+1)",
            expect![[r#"
Root@0..9
  AnonymousRecord@0..9
    Body@0..9
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      InfixExpr@2..9
        Literal@2..3
          Number@2..3 "5"
        Star@3..4 "*"
        ParenExpr@4..9
          LParen@4..5 "("
          InfixExpr@5..8
            Literal@5..6
              Number@5..6 "2"
            Plus@6..7 "+"
            Literal@7..8
              Number@7..8 "1"
          RParen@8..9 ")""#]],
        );
    }

    #[test]
    fn parse_nd_list() {
        check(
            "var: 1,2,3; a,b,c",
            expect![[r#"
Root@0..17
  Record@0..17
    Name@0..3
      Ident@0..3 "var"
    Colon@3..4 ":"
    Whitespace@4..5 " "
    Body@5..17
      List@5..17
        Row@5..12
          Entry@5..6
            Literal@5..6
              Number@5..6 "1"
          Comma@6..7 ","
          Entry@7..8
            Literal@7..8
              Number@7..8 "2"
          Comma@8..9 ","
          Entry@9..10
            Literal@9..10
              Number@9..10 "3"
          Semicolon@10..11 ";"
          Whitespace@11..12 " "
        Row@12..17
          Entry@12..13
            Ref@12..13
              Name@12..13
                Ident@12..13 "a"
          Comma@13..14 ","
          Entry@14..15
            Ref@14..15
              Name@14..15
                Ident@14..15 "b"
          Comma@15..16 ","
          Entry@16..17
            Ref@16..17
              Name@16..17
                Ident@16..17 "c""#]],
        );
    }

    #[test]
    fn parse_column_list() {
        check(
            "var: 1;2;3",
            expect![[r#"
Root@0..10
  Record@0..10
    Name@0..3
      Ident@0..3 "var"
    Colon@3..4 ":"
    Whitespace@4..5 " "
    Body@5..10
      List@5..10
        Row@5..7
          Entry@5..6
            Literal@5..6
              Number@5..6 "1"
          Semicolon@6..7 ";"
        Row@7..9
          Entry@7..8
            Literal@7..8
              Number@7..8 "2"
          Semicolon@8..9 ";"
        Row@9..10
          Entry@9..10
            Literal@9..10
              Number@9..10 "3""#]],
        );
    }

    #[test]
    fn parse_open_list_precedence() {
        check(
            "var: 1;2;3 ~> b",
            expect![[r#"
Root@0..15
  Record@0..15
    Name@0..3
      Ident@0..3 "var"
    Colon@3..4 ":"
    Whitespace@4..5 " "
    Body@5..15
      InfixExpr@5..15
        List@5..11
          Row@5..7
            Entry@5..6
              Literal@5..6
                Number@5..6 "1"
            Semicolon@6..7 ";"
          Row@7..9
            Entry@7..8
              Literal@7..8
                Number@7..8 "2"
            Semicolon@8..9 ";"
          Row@9..11
            Entry@9..11
              Literal@9..11
                Number@9..10 "3"
                Whitespace@10..11 " "
        RSquiggleArrow@11..13 "~>"
        Whitespace@13..14 " "
        Ref@14..15
          Name@14..15
            Ident@14..15 "b""#]],
        );
    }


    #[test]
    fn parse_unclosed_parentheses() {
        check(
            "var: (foo",
            expect![[r#"
Root@0..9
  Record@0..9
    Name@0..3
      Ident@0..3 "var"
    Colon@3..4 ":"
    Whitespace@4..5 " "
    Body@5..9
      ParenExpr@5..9
        LParen@5..6 "("
        Ref@6..9
          Name@6..9
            Ident@6..9 "foo"
error at 6..9: expected ., ::, +, -, *, /, %, .., &, |, ->, ~>, ->, ., :: or )"#]],
        );
    }

    #[test]
    fn parse_simple_disjunction() {
        check(
            r#"var: "A" + "B" + "C""#,
            expect![[r#"
Root@0..20
  Record@0..20
    Name@0..3
      Ident@0..3 "var"
    Colon@3..4 ":"
    Whitespace@4..5 " "
    Body@5..20
      InfixExpr@5..20
        InfixExpr@5..15
          Literal@5..9
            String@5..8 "\"A\""
            Whitespace@8..9 " "
          Plus@9..10 "+"
          Whitespace@10..11 " "
          Literal@11..15
            String@11..14 "\"B\""
            Whitespace@14..15 " "
        Plus@15..16 "+"
        Whitespace@16..17 " "
        Literal@17..20
          String@17..20 "\"C\"""#]],
        )
    }

    #[test]
    fn parse_struct_disjunction() {
        check(
            r#"var: {name: "frank"} | {name: "johnny"}"#,
            expect![[r#"
Root@0..39
  Record@0..39
    Name@0..3
      Ident@0..3 "var"
    Colon@3..4 ":"
    Whitespace@4..5 " "
    Body@5..39
      InfixExpr@5..39
        Struct@5..21
          LBrace@5..6 "{"
          Record@6..19
            Name@6..12
              Ident@6..10 "name"
              Colon@10..11 ":"
              Whitespace@11..12 " "
            Body@12..19
              Literal@12..19
                String@12..19 "\"frank\""
          RBrace@19..20 "}"
          Whitespace@20..21 " "
        Bar@21..22 "|"
        Whitespace@22..23 " "
        Struct@23..39
          LBrace@23..24 "{"
          Record@24..38
            Name@24..30
              Ident@24..28 "name"
              Colon@28..29 ":"
              Whitespace@29..30 " "
            Body@30..38
              Literal@30..38
                String@30..38 "\"johnny\""
          RBrace@38..39 "}""#]],
        )
    }

    #[test]
    fn parse_namespaced_expr() {
        check(
            r#"var: {
  field: <; @global::mixins::ctypes ;> -> $uint32
}"#,
            expect![[r#"
Root@0..58
  Record@0..58
    Name@0..3
      Ident@0..3 "var"
    Colon@3..4 ":"
    Whitespace@4..5 " "
    Body@5..58
      Struct@5..58
        LBrace@5..6 "{"
        Whitespace@6..9 "\n  "
        Record@9..57
          Name@9..16
            Ident@9..14 "field"
            Colon@14..15 ":"
            Whitespace@15..16 " "
          Body@16..57
            InfixExpr@16..57
              ScopeBlock@16..46
                List@16..46
                  LBird@16..18 "<;"
                  Whitespace@18..19 " "
                  Ref@19..43
                    Path@19..43
                      Keyword@19..26
                        At@19..20 "@"
                        Ident@20..26 "global"
                      ColonColon@26..28 "::"
                      Path@28..43
                        Name@28..34
                          Ident@28..34 "mixins"
                        ColonColon@34..36 "::"
                        Name@36..43
                          Ident@36..42 "ctypes"
                          Whitespace@42..43 " "
                  RBird@43..45 ";>"
                  Whitespace@45..46 " "
              RArrow@46..48 "->"
              Whitespace@48..49 " "
              Ref@49..57
                Schema@49..57
                  Name@49..57
                    DollarSign@49..50 "$"
                    Ident@50..56 "uint32"
                    Whitespace@56..57 "\n"
        RBrace@57..58 "}""#]],
        )
    }

    #[test]
    fn parse_struct_math() {
        check(
            "> {a:4b:5}+{a:2b:-5}",
            expect![[r#"
Root@0..20
  AnonymousRecord@0..20
    Body@0..20
      RAngleBrack@0..1 ">"
      Whitespace@1..2 " "
      InfixExpr@2..20
        Struct@2..10
          LBrace@2..3 "{"
          Record@3..6
            Name@3..5
              Ident@3..4 "a"
              Colon@4..5 ":"
            Body@5..6
              Literal@5..6
                Number@5..6 "4"
          Record@6..9
            Name@6..8
              Ident@6..7 "b"
              Colon@7..8 ":"
            Body@8..9
              Literal@8..9
                Number@8..9 "5"
          RBrace@9..10 "}"
        Plus@10..11 "+"
        Struct@11..20
          LBrace@11..12 "{"
          Record@12..15
            Name@12..14
              Ident@12..13 "a"
              Colon@13..14 ":"
            Body@14..15
              Literal@14..15
                Number@14..15 "2"
          Record@15..19
            Name@15..17
              Ident@15..16 "b"
              Colon@16..17 ":"
            Body@17..19
              PrefixExpr@17..19
                Minus@17..18 "-"
                Literal@18..19
                  Number@18..19 "5"
          RBrace@19..20 "}""#]],
        );
    }

    #[test]
    fn parse_transform_path() {
        check(
            r#"var: 32->#reduce -> some::module::path::#transform"#,
            expect![[r#"
Root@0..20
  Record@0..20
    Name@0..3
      Ident@0..3 "var"
    Colon@3..4 ":"
    Whitespace@4..5 " "
    Body@5..20
      InfixExpr@5..20
        InfixExpr@5..15
          Literal@5..9
            String@5..8 "\"A\""
            Whitespace@8..9 " "
          Plus@9..10 "+"
          Whitespace@10..11 " "
          Literal@11..15
            String@11..14 "\"B\""
            Whitespace@14..15 " "
        Plus@15..16 "+"
        Whitespace@16..17 " "
        Literal@17..20
          String@17..20 "\"C\"""#]],
        )
    }
}
