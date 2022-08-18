use std::panic;

use crate::{BinaryOp, HirExpr, HirRecord, HirStmt, UnaryOp};
use la_arena::Arena;
use smol_str::SmolStr;
use syntax::SyntaxKind;

#[derive(Debug, PartialEq, Default)]
pub struct Database {
    exprs: Arena<HirExpr>,
}

impl Database {
    pub(crate) fn lower_stmt(&mut self, ast: ast::Stmt) -> Option<HirStmt> {
        let result = match ast {

            ast::Stmt::Record(ast) => HirStmt::Record(self.lower_record(Some(ast))),

            // ast::Stmt::AnonRecord(ast) => HirStmt::Record(self.lower_anon_record(Some(ast))),

            ast::Stmt::Expr(ast) => HirStmt::Expr(self.lower_expr(Some(ast))),

            ast::Stmt::Accessor(ast) => {
                HirStmt::Accessor { 
                    name: ast.name()?.text().into(),
                    source: ast.path().unwrap().name().into_iter()
                        .map(|n| n.text().into())
                        .collect::<Vec<SmolStr>>()
                    // source: ast.value().iter()
                    //     .map(|s| s.text().into())
                    //     .collect::<Vec<SmolStr>>()
                }
            },

            _ => {
                println!("Well that was unexpected");
                panic!()
            }
        };

        Some(result)
    }

    pub(crate) fn lower_record(&mut self, ast: Option<ast::Record>) -> HirRecord {
        if let Some(ast) = ast {
            match ast {
                ast::Record::Mono(ast) => {
                    return HirRecord::Mono { 
                        name: ast.name(),
                        value: self.lower_expr(ast.value()),
                        constraint: Some(self.lower_expr(ast.constraint())),
                        scope: Some(self.lower_expr(ast.scope())),
                        schema: false,
                    };
                },
                ast::Record::Poly(ast) => {
                    return HirRecord::Poly { 
                        name: ast.name(),
                        value: ast.value().into_iter()
                            .filter_map(|v| Some(self.lower_record(Some(v))))
                            .collect::<Vec<HirRecord>>(),
                        constraint: Some(self.lower_expr(ast.constraint())),
                        scope: Some(self.lower_expr(ast.scope())),
                        schema: ast.is_schema(),
                    };
                },
                // _ => {return HirRecord::Missing}
            }

        } else {
            return HirRecord::Missing;
        }
    }

    pub(crate) fn lower_expr(&mut self, ast: Option<ast::Expr>) -> HirExpr {
        if let Some(ast) = ast {
            match ast {
                ast::Expr::BinaryExpr(ast) => self.lower_binary(ast),
                ast::Expr::Literal(ast) => HirExpr::Literal { n: ast.parse() },
                ast::Expr::ParenExpr(ast) => self.lower_expr(ast.expr()),
                ast::Expr::UnaryExpr(ast) => self.lower_unary(ast),
                ast::Expr::Ref(ast) => self.lower_variable_ref(ast),
                ast::Expr::List(ast) => self.lower_list(ast),
            }
        } else {
            HirExpr::Missing
        }
    }

    fn lower_binary(&mut self, ast: ast::BinaryExpr) -> HirExpr {
        println!("HERE: {:?}", ast.op());
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Plus      => BinaryOp::Add,
            SyntaxKind::Minus     => BinaryOp::Sub,
            SyntaxKind::Star      => BinaryOp::Mul,
            SyntaxKind::Slash     => BinaryOp::Div,
            SyntaxKind::Ampersand => BinaryOp::Unify,
            _ => unreachable!(),
        };

        let lhs = self.lower_expr(ast.lhs());
        let rhs = self.lower_expr(ast.rhs());

        HirExpr::Binary {
            op,
            lhs: self.exprs.alloc(lhs),
            rhs: self.exprs.alloc(rhs),
        }
    }

    fn lower_unary(&mut self, ast: ast::UnaryExpr) -> HirExpr {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Minus => UnaryOp::Neg,
            _ => unreachable!(),
        };

        let expr = self.lower_expr(ast.expr());

        HirExpr::Unary {
            op,
            expr: self.exprs.alloc(expr),
        }
    }

    fn lower_variable_ref(&mut self, ast: ast::Ref) -> HirExpr {
        HirExpr::Ref {
            var: ast.name().unwrap().text().into(),
        }
    }

    fn lower_list(&mut self, ast: ast::List) -> HirExpr {
        HirExpr::List { 
            items: ast.items().into_iter()
                .filter_map( 
                    |ast| 
                    Some(self.lower_expr(Some(ast)))
                ).collect() 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> ast::Root {
        ast::Root::cast(parser::parse(input).syntax()).unwrap()
    }

    fn check_stmt(input: &str, expected_hir: HirStmt) {
        let root = parse(input);
        println!("{}", parser::parse(input).debug_tree());
        let ast = root.stmts().next().unwrap();
        let hir = Database::default().lower_stmt(ast).unwrap();
        println!("{:#?}", hir);
        assert_eq!(hir, expected_hir);
    }

    // fn check_expr(input: &str, expected_hir: HirExpr, expected_database: Database) {
    //     let root = parse(input);
    //     let first_stmt = root.stmts().next().unwrap();
        
    //     println!("{}", parser::parse(input).debug_tree());
    //     let ast = match first_stmt {
    //         ast::Stmt::Expr(ast) => ast,
    //         _ => {
    //             println!("HOLY JESUS FUCKING CHRIST");
    //             println!("{:#?}", first_stmt);
    //             unreachable!()
    //         }
    //     };
    //     let mut database = Database::default();
    //     let hir = database.lower_expr(Some(ast));

    //     assert_eq!(hir, expected_hir);
    //     assert_eq!(database, expected_database);
    // }

    // TODO: Why does this work?
    #[test]
    fn lower_record_dec() {
        let rec = HirStmt::Record(
            HirRecord::Poly {
                name: Some("foo".into()),
                value: vec![
                    HirRecord::Mono { 
                        name: Some("name".into()),
                        value: HirExpr::Literal { n: Some(4) },
                        constraint: Some(HirExpr::Missing),
                        schema: false,
                        scope: Some(HirExpr::Missing),
                     }
                ],
                constraint: Some(HirExpr::Missing),
                schema: false,
                scope: Some(HirExpr::Missing),
            }
        );
        check_stmt(
            "foo: {name: 4}",
            rec,
        );
    }

    #[test]
    fn lower_constrained_record() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(HirExpr::Ref { var: "aSchema".into() });
        let rhs = exprs.alloc(HirExpr::Ref { var: "anotherSchema".into() } );
        let rcrd = HirStmt::Record(
            HirRecord::Poly { 
                name: Some("foo".into()),
                value: vec![
                    HirRecord::Mono {
                        name: Some("name".into()),
                        value: HirExpr::Literal { n: Some(4) },
                        constraint: Some(HirExpr::Missing),
                        schema: false,
                        scope: Some(HirExpr::Missing),
                    }
                ], 
                constraint: Some(HirExpr::Binary { 
                    op: BinaryOp::Unify,
                    lhs: lhs,
                    rhs: rhs 
                }),
                schema: false,
                scope: Some(HirExpr::Missing),
            }
        );
        check_stmt(
            "foo: aSchema & anotherSchema {name: 4}",
            rcrd
        );
    }

    #[test]
    fn lower_empty_record() {
        check_stmt(
            "foo: {}",
            HirStmt::Record(
                HirRecord::Poly { 
                    name: Some("foo".into()),
                    value: vec![],
                    constraint: Some(HirExpr::Missing),
                    schema: false,
                    scope: Some(HirExpr::Missing),
                }
            )
        );
    }

    #[test]
    fn lower_accessor_declaration() {
        check_stmt(
            "#zalgo: @global::zalgo",
            HirStmt::Accessor {
                name: "zalgo".into(),
                source: vec!["@".into(), "global".into(), "zalgo".into()]
            }
        );
    }

    // This test isn't valid, since Vada doesn't have a clear token to reset on.
    // We'll need some sort of semi-predictive way to eay through tokens
    // after an error in order to keep it from leaking forward.
    // #[test]
    // fn lower_variable_def_without_name() {
    //     let root = parse("> :10");
    //     let ast = root.stmts().next().unwrap();
    //     println!("{}", parser::parse(":10").debug_tree());

    //     assert!(Database::default().lower_stmt(ast).is_none());
    // }


    #[test]
    fn lower_empy_record() {
        check_stmt(
            "a:",
            HirStmt::Record(
                HirRecord::Mono { 
                    name: Some("a".into()),
                    value: HirExpr::Missing,
                    constraint: Some(HirExpr::Missing),
                    schema: false,
                    scope: Some(HirExpr::Missing), 
                }
            )

        );
    }

    #[test]
    fn lower_expr_stmt() {
        check_stmt(
            ">123",
            HirStmt::Record(
                HirRecord::Mono {
                    name: None, 
                    value: HirExpr::Literal { n: Some(123) },
                    constraint: Some(HirExpr::Missing),
                    scope: Some(HirExpr::Missing),
                    schema: false,
                }
            )
        );
    }

    #[test]
    fn lower_binary_expr() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(HirExpr::Literal { n: Some(1) });
        let rhs = exprs.alloc(HirExpr::Literal { n: Some(2) });

        check_stmt(
            "> 1+ 2",
            HirStmt::Record(
                HirRecord::Mono { 
                    name: None,
                    value: HirExpr::Binary {
                        lhs,
                        rhs,
                        op: BinaryOp::Add,
                    },
                constraint: Some(HirExpr::Missing),
                scope: Some(HirExpr::Missing),
                schema: false,
                }
            )
        );
    }

    #[test]
    fn lower_constraint_expr() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(HirExpr::Literal { n: Some(1) });
        let rhs = exprs.alloc(HirExpr::Literal { n: Some(2) });

        check_stmt(
            "> 1 & 2",
            HirStmt::Record(
                HirRecord::Mono {
                    name: None,
                    value: HirExpr::Binary {
                        lhs,
                        rhs,
                        op: BinaryOp::Unify,
                    },
                    constraint: Some(HirExpr::Missing),
                    scope: Some(HirExpr::Missing),
                    schema: false
                }
            )
        );
    }

    #[test]
    fn lower_binary_expr_without_rhs() {
        let mut exprs = Arena::new();
        let lhs = exprs.alloc(HirExpr::Literal { n: Some(10) });
        let rhs = exprs.alloc(HirExpr::Missing);

        check_stmt(
            "> 10 -",
            HirStmt::Record(
                HirRecord::Mono {
                    name: None, 
                    value: HirExpr::Binary {
                        lhs,
                        rhs,
                        op: BinaryOp::Sub,
                    },
                    constraint: Some(HirExpr::Missing),
                    scope: Some(HirExpr::Missing),
                    schema: false,
                }
            )
        );
    }

    #[test]
    fn lower_literal() {
        check_stmt(
            "> 999",
            HirStmt::Record(
                HirRecord::Mono { 
                    name: None,
                    value: HirExpr::Literal { n: Some(999) },
                    constraint: Some(HirExpr::Missing),
                    scope: Some(HirExpr::Missing),
                    schema: false
                }
            )
        );
    }

    #[test]
    fn lower_paren_expr() {
        check_stmt(
            "> ((((((abc))))))",
            HirStmt::Record(
                HirRecord::Mono {
                    name: None, 
                    value: HirExpr::Ref { var: "abc".into() },
                    constraint: Some(HirExpr::Missing),
                    scope: Some(HirExpr::Missing),
                    schema: false
                }
            )
        );
    }

    #[test]
    fn lower_unary_expr() {
        let mut exprs = Arena::new();
        let ten = exprs.alloc(HirExpr::Literal { n: Some(10) });
        check_stmt(
            "> -10",
            HirStmt::Record(
                HirRecord::Mono {
                    name: None, 
                    value: HirExpr::Unary {
                        expr: ten,
                        op: UnaryOp::Neg,
                    },
                    constraint: Some(HirExpr::Missing),
                    scope: Some(HirExpr::Missing),
                    schema: false
                }
            )
        );
    }

    #[test]
    fn lower_unary_expr_without_expr() {
        let mut exprs = Arena::new();
        let expr = exprs.alloc(HirExpr::Missing);

        check_stmt(
            "> -",
            HirStmt::Record(
                HirRecord::Mono {
                    name: None,
                    value: HirExpr::Unary {
                        expr,
                        op: UnaryOp::Neg,
                    },
                    constraint: Some(HirExpr::Missing),
                    scope: Some(HirExpr::Missing),
                    schema: false
                }
            )
        );
    }

    #[test]
    fn lower_variable_ref() {
        check_stmt(
            "> foo",
            HirStmt::Record(
                HirRecord::Mono {
                    name: None,
                    value: HirExpr::Ref { var: "foo".into() },
                    constraint: Some(HirExpr::Missing),
                    scope: Some(HirExpr::Missing),
                    schema: false,
                }
            )
        );
        // check_expr(
        //     ">foo",
        //     HirExpr::Ref { var: "foo".into() },
        //     Database::default(),
        // );
    }

    #[test]
    fn lower_accessor() {
        check_stmt(
            "#zalgo: @global::zalgo", 
            HirStmt::Accessor { 
                name: "zalgo".into(),
                source: vec![
                    "@".into(),
                    "global".into(),
                    "zalgo".into()] 
            }
        );
    }

    #[test]
    fn lower_pathed_accessor() {
        check_stmt(
            "#zalgo: @local::zalgo::fiddle::fuck.thing", 
            HirStmt::Accessor { 
                name: "zalgo".into(),
                source: vec![
                    "@".into(),
                    "local".into(),
                    "zalgo".into(),
                    "fiddle".into(),
                    "fuck".into(),
                    "thing".into()
                ] }
        );
    }
}