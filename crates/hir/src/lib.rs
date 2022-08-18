mod database;
pub use database::Database;

use la_arena::Idx;
use smol_str::SmolStr;

type ExprIdx = Idx<HirExpr>;

#[derive(Debug, PartialEq)]
pub enum HirStmt {
    // Record   { name: SmolStr, schema: bool, constraint: Option<HirExpr>, value: RecordBody },
    Record(HirRecord),
    Field    { name: SmolStr, value: HirExpr},
    Expr(HirExpr),
    Accessor { name: SmolStr, source: Vec<SmolStr>}
}

#[derive(Debug, PartialEq)]
pub enum HirRecord {
    Missing,
    Mono {
        name: Option<SmolStr>,
        value: HirExpr,
        constraint: Option<HirExpr>,
        scope: Option<HirExpr>,
        schema: bool
    },
    Poly {
        name: Option<SmolStr>,
        value: Vec<HirRecord>,
        constraint: Option<HirExpr>,
        scope: Option<HirExpr>,
        schema: bool
    },
}

#[derive(Debug, PartialEq)]
pub enum RecordBody {
    Missing,
    MonoRecord {
        // name: SmolStr,
        exp: HirExpr,
    },
    PolyRecord {
        // name: SmolStr,
        value: Vec<Field>
    }
}

#[derive(Debug, PartialEq)]
pub enum HirExpr {
    Missing,
    Binary {
        op: BinaryOp,
        lhs: ExprIdx,
        rhs: ExprIdx,
    },
    Literal {
        /// is `None` if the number is too big to fit in a u64
        n: Option<u64>,
    },
    Unary {
        op: UnaryOp,
        expr: ExprIdx,
    },
    Ref {
        var: SmolStr,
    },
    Struct {
        fields: Vec<Field>
    },
    List {
        items: Vec<HirExpr>
    }
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Unify,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, PartialEq)]
pub enum Field {
    MissingField,
    AnonymousField,
    NamedField {
        name: SmolStr,
        value: HirExpr
    },
}

#[derive(Debug, PartialEq)]
pub struct Path {
    root: SmolStr,
    path: Vec<SmolStr>
}

pub fn lower(ast: ast::Root) -> (Database, Vec<HirStmt>) {
    let mut db = Database::default();
    let stmts = ast.stmts().filter_map(|stmt| db.lower_stmt(stmt)).collect();

    (db, stmts)
}