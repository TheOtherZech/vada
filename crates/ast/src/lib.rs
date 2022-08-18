pub mod validation;

use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};
use smol_str::SmolStr;

#[derive(Debug)]
pub struct Root(SyntaxNode);

impl Root {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::Root {
            Some(Self(node))
        } else {
            println!("False Root. This should never happen");
            None
        }
    }

    pub fn stmts(&self) -> impl Iterator<Item = Stmt> {
        self.0.children().filter_map(Stmt::cast)
    }
}

#[derive(Debug)]
pub enum Stmt {
    ImportStmt(ImportStmt),
    Accessor(Accessor),
    Record(Record),
    AnonRecord(AnonRecord),
    Struct(Struct),
    Expr(Expr),
    // Type(Type),
}

impl Stmt {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::ImportStmt      => Self::ImportStmt(ImportStmt(node)),
            SyntaxKind::Accessor        => Self::Accessor(Accessor(node)),
            SyntaxKind::Record          => Self::Record(Record::cast(node)?),
            SyntaxKind::AnonymousRecord => Self::Record(Record::cast(node)?),
            SyntaxKind::Schema          => Self::Record(Record::cast(node)?),
            SyntaxKind::Struct          => Self::Struct(Struct(node)),
            // SyntaxKind::Type         => Self::Type(Type),
            _                           => Self::Expr(Expr::cast(node)?),
        };

        Some(result)
    }
}

#[derive(Debug)]
pub struct ImportStmt(SyntaxNode);

impl ImportStmt {
    // TODO: This 
}

#[derive(Debug)]
pub struct Accessor(SyntaxNode);

impl Accessor {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find(|node| node.kind() == SyntaxKind::Name).unwrap()
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn path(&self) -> Option<Path> {
        self.0.children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find_map(Path::cast)
    }

    pub fn value(&self) -> Vec<SyntaxToken> {
        return self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find(|node| node.kind() == SyntaxKind::Path).unwrap()
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .collect::<Vec<SyntaxToken>>()
    }
}

#[derive(Debug)]
pub struct AnonRecord(SyntaxNode);

impl AnonRecord {
    pub fn name(&self) -> String {
        return "".to_string()
    }

    pub fn constraint(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn is_schema(&self) -> bool {
        return false;
    }

    pub fn value(&self) -> Option<RecordBody> {
        return self.0.children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find(|node| node.kind() == SyntaxKind::Body)?
            .children()
            .find_map(RecordBody::cast);
    }

}

#[derive(Debug)]
pub enum Record {
    Mono(MonoRec),
    Poly(PolyRec),
}

impl Record {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if let Some(body) = node.children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find(|n| n.kind() == SyntaxKind::Body) {
                match body.first_child()?.kind() {
                    SyntaxKind::Struct => {
                        println!("Polyrecord");
                        return Some(Self::Poly(PolyRec(node)))
                    }
                    // TODO: Wrap this shit
                    SyntaxKind::Expr
                    | SyntaxKind::InfixExpr
                    | SyntaxKind::ParenExpr
                    | SyntaxKind::PrefixExpr
                    | SyntaxKind::ConstraintExpr
                    | SyntaxKind::Ref
                    | SyntaxKind::Literal   =>  {
                        println!("MONORECORD");
                        return Some(Self::Mono(MonoRec(node)))
                    }
                    _ => {
                        println!("hard fail {:?}", body.kind());
                        return None
                    }
                }
            }
            else {
                // Even if the record is malformed, we need to return
                // SOMETHING with a body.
                println!("Returning Malformed MonoRec");
                return Some(Self::Mono(MonoRec(node)));
            }
        }

}

#[derive(Debug)]
pub struct MonoRec(SyntaxNode);

impl MonoRec {
    pub fn name(&self) -> Option<SmolStr> {
        if let Some(raw) = self.raw_name() {
            return Some(raw.text().into());
        } else {
            return None;
        }
    }
    fn raw_name(&self) -> Option<SyntaxToken> {
        if let Some(n) = self.0.children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find(|node| node.kind() == SyntaxKind::Name) 
        {
            return n.children_with_tokens()
                .filter_map(SyntaxElement::into_token)
                .find(|token| token.kind() == SyntaxKind::Ident);
        } else {
            return None;
        }
    }

    pub fn value(&self) -> Option<Expr> {
        if let Some(n) = self.0.children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find(|node| node.kind() == SyntaxKind::Body) 
        {
            return n.children().find_map(Expr::cast);
        } else {
            return None;
        }
    }

    pub fn constraint(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn is_schema(&self) -> bool {
        self.0.kind() == SyntaxKind::Schema
    }

    pub fn scope(&self) -> Option<Expr> {
        if let Some(s) = self.0.children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find(|node| node.kind() == SyntaxKind::ScopeBlock) {
                return Expr::cast(s)
        }
        else {
            return None;
        }
    }
}

#[derive(Debug)]
pub struct PolyRec(SyntaxNode);

impl PolyRec {
    pub fn name(&self) -> Option<SmolStr> {
        if let Some(raw) = self.raw_name() {
            return Some(raw.text().into());
        } else {
            return None;
        }
    }

    fn raw_name(&self) -> Option<SyntaxToken> {
        if let Some(n) = self.0.children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find(|node| node.kind() == SyntaxKind::Name) 
        {
            return n.children_with_tokens()
                .filter_map(SyntaxElement::into_token)
                .find(|token| token.kind() == SyntaxKind::Ident);
        } else {
            return None;
        }
    }

    pub fn value(&self) -> Vec<Record> {
        if let Some(n) = self.0.children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find(|node| node.kind() == SyntaxKind::Body)
        {
            if let Some(nn) = n.children_with_tokens()
                .filter_map(SyntaxElement::into_node)
                .find(|node| node.kind() == SyntaxKind::Struct) {
                    println!("thunk");
                    return nn.children_with_tokens()
                        .filter_map(SyntaxElement::into_node)
                        .filter_map(Record::cast)
                        .collect::<Vec<Record>>();
                } else {
            return Vec::new();
        }

        } else {
            return Vec::new();
        }
    }

    pub fn constraint(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn is_schema(&self) -> bool {
        self.0.kind() == SyntaxKind::Schema
    }

    pub fn scope(&self) -> Option<Expr> {
        if let Some(s) = self.0.children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find(|node| node.kind() == SyntaxKind::ScopeBlock) {
                return Expr::cast(s)
        }
        else {
            return None;
        }
    }
}

#[derive(Debug)]
pub enum RecordBody {
    Field(Field),
    Struct(Struct),
    Expr(Expr),
    Missing
}

impl RecordBody {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::Struct      => Self::Struct(Struct(node)),
            SyntaxKind::Field       => Self::Field(Field(node)),
            SyntaxKind::InfixExpr   => Self::Expr(Expr::cast(node)?),
            SyntaxKind::PrefixExpr  => Self::Expr(Expr::cast(node)?),
            SyntaxKind::Literal     => Self::Expr(Expr::cast(node)?),
            SyntaxKind::ParenExpr   => Self::Expr(Expr::cast(node)?),
            SyntaxKind::Ref         => Self::Expr(Expr::cast(node)?),
            _ => {
                println!("I AM THE GODHEAD {:?}", node.kind());
                return None
            }
        };

        return Some(result);
    }
}

#[derive(Debug)]
pub struct Struct(SyntaxNode);

impl Struct {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    // TODO: Should this be an arena?
    pub fn value(&self) -> Option<Vec<Record>> {
        println!("{:#?}", self.0.children());
        return Some(
            self.0.children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .filter_map(|node| Record::cast(node))
            .collect::<Vec<Record>>()
        );
    }

    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::Struct => Self(node),
            _ => return None
        };

        return Some(result);
    }
}

#[derive(Debug)]
pub struct Field(SyntaxNode);

impl Field {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn value(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::Field => Self(node),
            _ => return None,
        };

        Some(result)
    }
}

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    Literal(Literal),
    List(List),
    ParenExpr(ParenExpr),
    UnaryExpr(UnaryExpr),
    Ref(Ref),
}

impl Expr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::InfixExpr      => Self::BinaryExpr(BinaryExpr(node)),
            SyntaxKind::Literal        => Self::Literal(Literal(node)),
            SyntaxKind::ParenExpr      => Self::ParenExpr(ParenExpr(node)),
            SyntaxKind::PrefixExpr     => Self::UnaryExpr(UnaryExpr(node)),
            SyntaxKind::Ref            => Self::Ref(Ref(node)),
            SyntaxKind::Type           => Self::Ref(Ref(node)),
            SyntaxKind::Transform      => Self::Ref(Ref(node)),
            SyntaxKind::ConstraintExpr => Self::cast(node.first_child().unwrap())?,
            SyntaxKind::List           => Self::List(List(node)),
            _ => {
                println!("EXPR CAST FAIL {:?}", node.kind());
                return None;
            }
        };

        Some(result)
    }
}

#[derive(Debug)]
pub struct BinaryExpr(SyntaxNode);

impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.0.children().filter_map(Expr::cast).nth(1)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        println!("Binary Ops:{:?}",
            self.0
                .children_with_tokens()
                .filter_map(SyntaxElement::into_token)
                .collect::<Vec<SyntaxToken>>()
                // .nth(1)
        );

        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .nth(0)
    }
}

#[derive(Debug)]
pub struct List(SyntaxNode);

impl List {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::List {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn items(&self) -> Vec<Expr> {
        self.0.children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .filter_map(Expr::cast)
            .collect()
    }


}

#[derive(Debug)]
pub struct Literal(SyntaxNode);

impl Literal {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::Literal {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn raw(&self) -> String {
        self.0.first_token().unwrap().text().to_string()
    }

    pub fn parse(&self) -> Option<u64> {
        self.0.first_token().unwrap().text().parse().ok()
    }
}

#[derive(Debug)]
pub struct ParenExpr(SyntaxNode);

impl ParenExpr {
    pub fn expr(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}

#[derive(Debug)]
pub struct UnaryExpr(SyntaxNode);

impl UnaryExpr {
    pub fn expr(&self) -> Option<Expr> {
        if self.0.first_child_or_token().unwrap().into_token().is_some() {
            self.0.children().find_map(Expr::cast)
        } else {
            self.0.children().skip(1).find_map(Expr::cast)
        }
    }

    pub fn op_but_better(&self) -> Vec<SyntaxToken> {
        if self.0.first_child_or_token().unwrap().into_token().is_some() {
            vec![self.0.first_token().unwrap()]
        }
        else {
            self.0.first_child().unwrap()
                .descendants_with_tokens()
                .filter_map(SyntaxElement::into_node)
                .filter(|n| n.kind() == SyntaxKind::Name)
                .map(
                    |c|
                    c.children_with_tokens()
                    .filter_map(SyntaxElement::into_token)
                    .filter(|t| t.kind() != SyntaxKind::Whitespace)
                    .collect::<Vec<SyntaxToken>>()
                )
                .fold(
                    Vec::new(),
                    |mut acc, i| {
                        acc.extend(i);
                        acc
                    }
                    
                )
        }
    }

    // TODO: Expand
    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| {
                matches!(
                    token.kind(),
                    SyntaxKind::Minus | SyntaxKind::Octothorpe
                    | SyntaxKind::At | SyntaxKind::DollarSign
                    | SyntaxKind::RAngleBrack | SyntaxKind::GreaterEqual
                    | SyntaxKind::LAngleBrack | SyntaxKind::LessEqual
                )
            })
    }
}

#[derive(Debug)]
pub struct Ref(SyntaxNode);

impl Ref {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0.first_token()
    }

    pub fn full_name(&self) -> Vec<SyntaxToken> {
        self.0.descendants_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .filter(|n| n.kind() == SyntaxKind::Name)
            .map(
                |c|
                c.children_with_tokens()
                .filter_map(SyntaxElement::into_token)
                .filter(|t| t.kind() != SyntaxKind::Whitespace)
                .collect::<Vec<SyntaxToken>>()
            )
            .fold(
                Vec::new(),
                |mut acc, i| {
                    acc.extend(i);
                    acc
                }
                
            )
    }

    pub fn path(&self) -> Option<Path> {
        self.0.children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            
            .find_map(Path::cast)
    }
}

#[derive(Debug)]
pub struct Path(SyntaxNode);

impl Path {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::Path {
            return Some(Self(node))
        } else {
            return None;
        }
    }

    pub fn path(&self) -> Option<Path> {
        self.0.children_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .find_map(Path::cast)
    }

    pub fn name(&self) -> Vec<SyntaxToken> {

        self.0.descendants_with_tokens()
            .filter_map(SyntaxElement::into_node)
            .filter(|n| n.kind() == SyntaxKind::Name)
            .map(
                |c|
                c.children_with_tokens()
                .filter_map(SyntaxElement::into_token)
                .filter(|t| t.kind() != SyntaxKind::Whitespace)
                .collect::<Vec<SyntaxToken>>()
            )
            .fold(
                Vec::new(),
                |mut acc, i| {
                    acc.extend(i);
                    acc
                }
                
            )

        // self.0.children_with_tokens()
        //     .filter_map(SyntaxElement::into_node)
        //     .find(|n| n.kind() == SyntaxKind::Path)
        //     .unwrap()
        //     .children_with_tokens()
        //     .filter_map(SyntaxElement::into_token)
        //     .filter(|t| t.kind() != SyntaxKind::Whitespace)
        //     .collect()
    }

    pub fn full_name(&self) {

    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    fn recurse_expression(exp: crate::Expr) -> String {
        let base = match exp {
            crate::Expr::Literal(exp) => {
                exp.raw().to_string()
            },
            crate::Expr::ParenExpr(exp) => {
                format!("{:}", &recurse_expression(exp.expr().unwrap())[..])
            },
            crate::Expr::UnaryExpr(exp) => {
                format!("[ {:} {:} ]",
                    exp.op_but_better().into_iter().fold(
                        "".to_string(),
                        |acc: String, i|
                        acc + i.text()
                    ),
                    &recurse_expression(exp.expr().unwrap())[..]
                )
            },
            crate::Expr::BinaryExpr(exp) => {
                // println!("{:?}", exp);
                match exp.op().unwrap().text() {
                    "~>" => format!("{:} \n     {:} {:}",
                                    &recurse_expression(exp.lhs().unwrap())[..],
                                    exp.op().unwrap().text(),
                                    &recurse_expression(exp.rhs().unwrap())[..],
                                    ),
                    "::" => format!("{:}{:}{:}",
                                    &recurse_expression(exp.lhs().unwrap())[..],
                                    exp.op().unwrap().text(),
                                    &recurse_expression(exp.rhs().unwrap())[..],
                                    ),
                    "." => format!("{:}{:}{:}",
                                    &recurse_expression(exp.lhs().unwrap())[..],
                                    exp.op().unwrap().text(),
                                    &recurse_expression(exp.rhs().unwrap())[..],
                                    ),
                    _ => format!("( {:} {:} {:} )",
                                    &recurse_expression(exp.lhs().unwrap())[..],
                                    exp.op().unwrap().text(),
                                    &recurse_expression(exp.rhs().unwrap())[..],
                                    ),
                }
            },
            crate::Expr::Ref(exp) => {
                if exp.path().is_some() {
                    exp.path().unwrap()
                        .name().into_iter()
                        .fold(
                            "".to_string(),
                            |acc: String, i| 
                            acc + i.text()
                        )
                }
                else {
                    exp.full_name().into_iter()
                        .fold(
                            "".to_string(),
                            |acc: String, i| 
                            acc + i.text()
                        )
                }
            },
            crate::Expr::List(exp) => {
                format!("[{}]", 
                    exp.items().into_iter()
                        .map(|e| recurse_expression(e))
                        .collect::<Vec<String>>()
                        .join(", ")
                    )
            } 
        };

        return base.to_string();
    }

    fn check_expression_associativity(input: &str, expected: &str) {
        println!("{}", parser::parse(input).debug_tree());
        let root = crate::Root::cast(parser::parse(input).syntax()).unwrap();
        let ast = root.stmts().next().unwrap();
        
        let record_body = match ast {
            crate::Stmt::Record(ast) => {
                match ast {
                    crate::Record::Mono(ast) => {
                        ast.value()
                    }
                    _ => {
                        println!("asd");
                        panic!()
                    }
                }
            }
            _ => {
                println!("SHIT");
                panic!()
            }
        };
        if record_body.is_some() {
            let out = recurse_expression(record_body.unwrap());
            println!("Output: {}", out);
            assert_eq!(out, expected);
        } else {
            println!("THIS SHOULDN'T HAPPEN HERE AT ALL EVER");
        }

        // let out = recurse_expression(record_body.unwrap());

        // println!("{}", out);
        // assert_eq!(out, expected);
    }

    #[test]
    fn validate_disconj_expr() {
        check_expression_associativity(
            r#"var: (1 + 2 | 3 * 4 | 5 + 6) & >= 3"#,
        " (( ( (1 + 2)  |  ( (3 * 4)  |  (5 + 6) ) ) ) & [>=3]) "
        )
    }
    #[test]
    fn validate_string_disjunction_expr() {
        check_expression_associativity(
            r#"var: "A" | "B" | "C" | "D""#,
        r#" ("A" |  ("B" |  ("C" | "D") ) ) "#
        )
    }

    #[test]
    fn validate_type_disjuction_expr() {
        check_expression_associativity(
            r#"var: $int & >=5 | $float & <= 5"#,
        r#" ( ($int & [>=5])  |  ($float & [<=5]) ) "#
        )
    }

    #[test]
    fn validate_fuck() {
        check_expression_associativity(
            r#"var: <=10 & <4+1"#,
        r#" ([<=10] & [< (4 + 1) ]) "#
        )
    }

    #[test]
    fn validate_fuck_harder() {
        check_expression_associativity(
            r#"var: <= 10 + 5 + 2"#,
        r#"[<= ( (10 + 5)  + 2) ]"#
        )
    }

    #[test]
    fn validate_add_negative() {
        check_expression_associativity(
            r#"var: 4 + -5"#,
        r#" (4 + [-5]) "#
        )
    }

    #[test]
    fn validate_add_negativer() {
        check_expression_associativity(
            r#"var: >4 & <5+2"#,
        r#" ([>4] & [< (5 + 2) ]) "#
        )
    }

    #[test]
    fn validate_arrow_expression() {
        check_expression_associativity(
            r#"var: 3 -> $uint"#,
        r#" (3 -> $uint) "#
        )
    }

    #[test]
    fn validate_arrow_expression_list() {
        check_expression_associativity(
            r#"var: [x,y,z] -> $uint"#,
        r#" ([x, y, z] -> $uint) "#
        )
    }

    #[test]
    fn validate_arrow_expression_list_complex() {
        check_expression_associativity(
            r#"var: [4+8, $string, 0..6] -> $uint"#,
        r#" ([ (4 + 8) , $string,  (0 .. 6) ] -> $uint) "#
        )
    }

    #[test]
    fn validate_seq_simple() {
        check_expression_associativity(
            r#"var: 4+8 ~> $uint"#,
            r#" ( (4 + 8)  ~> $uint) "#)
    }

    #[test]
    fn validate_string_slice() {
        check_expression_associativity(
            r##"var: "elbow".[0..2] "##,
            r#" ("elbow" . [ (0 .. 2) ]) "#)
    }

    #[test]
    fn validate_type_method() {
        check_expression_associativity(
            r##"var: "elbow" -> #TitleCase "##,
            r#" ("elbow" -> #TitleCase) "#)
    }

    #[test]
    fn wasdd() {
        check_expression_associativity(
            r##"var: 32->#reduce: 4 + 5 ~> some::module::path::#transform(2*2) "##,
            r#" ("elbow" -> #TitleCase) "#)
    }


    #[test]
    fn wasdder() {
        check_expression_associativity(
            r##"var: -5 + 2 * 4 - 2 / 4 "##,
            r#" ("elbow" -> #TitleCase) "#)
    }

    #[test]
    fn seq_format_test() {
        check_expression_associativity(
            r##"var: 2+3*4-2 ~> 4+6-3*8 ~> 4/2/2/2/4 ~> 2*2"##,
            r#" ("elbow" -> #TitleCase) "#)
    }

}

