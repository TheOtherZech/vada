use lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

/// TL;DR: Lexer tokens + high-level ast nodes (fields, records, etc)
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
    // Token-like Types
    Whitespace,
    Ident,

    // Literals
    Literal,
    Number,
    BinaryNumber,
    OctalNumber,
    HexNumber,
    AddressLiteral,
    IntLiteral,
    RandIntLiteral,
    FloatLiteral,
    RandFloatLiteral,
    String,

    // Tokens
    Backtick,
    Tilde,
    Bang,
    At,
    Octothorpe,
    DollarSign,
    Percent,
    Carrot,
    Ampersand,
    Star,
    LParen,
    RParen,
    Minus,
    Underscore,
    Equals,
    Plus,
    LBrack,
    RBrack,
    LBrace,
    RBrace,
    Backslash,
    Bar,
    Semicolon,
    Colon,
    SingleQuote,
    DoubleQuote,
    Comma,
    Dot,
    LAngleBrack,
    RAngleBrack,
    Slash,
    QMark,
    Diamond,

    // Digraphs
    ColonColon,
    NotEqual,
    LessEqual,
    GreaterEqual,
    PatternEqual,
    NotPattern,

    Walrus,
    Mustache,

    LArrow,
    RArrow,
    LSquiggleArrow,
    RSquiggleArrow,
    LBird,
    RBird,

    LJuke,
    RJuke,
    LDee,
    RDee,

    DotDot,
    DotDotDot,

    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    BitShiftLeft,
    BitShiftRight,

    LitTilde,
    LitBang,
    LitAt,
    LitOctothorpe,
    LitDollarSign,
    LitPercent,
    LitCarrot,
    LitAmpersand,
    LitStar,
    LitMinus,
    LitEqual,
    LitPlus,
    LitUnderscore,
    LitColon,
    LitDot,
    LitSlash,

    DSLDelimiter,
    Bottom,
    SectionMarker,

    // Compound Types
    Comment,
    Error,
    Root,

    // Expressions
    // The Expr type is a thing wrapper for the other types
    Expr, 
    InfixExpr,
    ParenExpr,
    PrefixExpr,
    ConstraintExpr,
    FilterExpr, // multi-accessor

    Struct,
    Section,
    ScopeBlock,

    // List Stuff
    Array,  // An Enclosed List? [1,2,3]
    Vec,
    List,  // 
    Entry, // 
    Row,

    Ref,

    // Record Elements
    Record,
    Name,
    Body,

    InlinedRecord,
    AnonymousRecord,
    Schema, 
    Field, // depreciate?

    // Do I need these?
    FuncCall,
    FuncArgs,

    // Accessor Stuff
    Accessor,
    Path,

    // Directives, AKA fancy keywords
    Directive,
    TypePrimitive,
    Keyword,
    Transform,

    Decorator, // depreciate?
    Type,
    UserType, // depreciate?

    // Top-Level
    ImportStmt, // depreciate?

    // This one should never show up in valid code
    TombStone,

    EOF,
}

impl SyntaxKind {
    pub fn raw(self) -> Option<u16> {
        return self.to_u16()
    }
}

impl From<TokenKind> for SyntaxKind {
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::Whitespace   => Self::Whitespace,
            TokenKind::Ident        => Self::Ident,

            // I'll likely need more SyntaxKinds for these.
            TokenKind::BinaryLiteral    => Self::BinaryNumber,
            TokenKind::OctalLiteral     => Self::OctalNumber,
            TokenKind::HexLiteral       => Self::HexNumber,
            TokenKind::AddressLiteral   => Self::AddressLiteral,
            TokenKind::IntLiteral       => Self::Number,
            TokenKind::RandIntLiteral   => Self::Number,
            TokenKind::FloatLiteral     => Self::Number,
            TokenKind::RandFloatLiteral => Self::Number,

            // Single-character tokens
            TokenKind::String       => Self::String,
            TokenKind::Backtick     => Self::Backtick,
            TokenKind::Tilde        => Self::Tilde,
            TokenKind::Bang         => Self::Bang,
            TokenKind::At           => Self::At,
            TokenKind::Octothorpe   => Self::Octothorpe,
            TokenKind::DollarSign   => Self::DollarSign,
            TokenKind::Percent      => Self::Percent,
            TokenKind::Carrot       => Self::Carrot,
            TokenKind::Ampersand    => Self::Ampersand,
            TokenKind::Star         => Self::Star,
            TokenKind::LParen       => Self::LParen,
            TokenKind::RParen       => Self::RParen,
            TokenKind::Minus        => Self::Minus,
            TokenKind::Underscore   => Self::Underscore,
            TokenKind::Equals       => Self::Equals,
            TokenKind::Plus         => Self::Plus,
            TokenKind::LBrack       => Self::LBrack,
            TokenKind::RBrack       => Self::RBrack,
            TokenKind::LBrace       => Self::LBrace,
            TokenKind::RBrace       => Self::RBrace,
            TokenKind::Backslash    => Self::Backslash,
            TokenKind::Bar          => Self::Bar,
            TokenKind::Semicolon    => Self::Semicolon,
            TokenKind::Colon        => Self::Colon,
            TokenKind::SingleQuote  => Self::SingleQuote,
            TokenKind::DoubleQuote  => Self::DoubleQuote,
            TokenKind::Comma        => Self::Comma,
            TokenKind::Dot          => Self::Dot,
            TokenKind::LAngleBrack  => Self::LAngleBrack,
            TokenKind::RAngleBrack  => Self::RAngleBrack,
            TokenKind::Slash        => Self::Slash,
            TokenKind::QMark        => Self::QMark,
            TokenKind::Diamond      => Self::Diamond,
            TokenKind::Comment      => Self::Comment,

            // Digraphs
            TokenKind::ColonColon     => Self::ColonColon,
            TokenKind::NotEqual       => Self::NotEqual,
            TokenKind::LessEqual      => Self::LessEqual,
            TokenKind::GreaterEqual   => Self::GreaterEqual,
            TokenKind::PatternEqual   => Self::PatternEqual,
            TokenKind::NotPattern     => Self::NotPattern,

            // Assignment Digraphs
            TokenKind::Walrus         => Self::Walrus,
            TokenKind::Mustache       => Self::Mustache,

            // Arrow Digraphs
            TokenKind::RArrow         => Self::RArrow,
            TokenKind::LSquiggleArrow => Self::LSquiggleArrow,
            TokenKind::RSquiggleArrow => Self::RSquiggleArrow,
            TokenKind::LBird          => Self::LBird,
            TokenKind::RBird          => Self::RBird,

            TokenKind::DotDot         => Self::DotDot,
            TokenKind::DotDotDot      => Self::DotDotDot,

            TokenKind::LJuke          => Self::LJuke,
            TokenKind::RJuke          => Self::RJuke,
            TokenKind::LDee           => Self::LDee,
            TokenKind::RDee           => Self::RDee,

            // Bit Op Digraphs
            TokenKind::BitAnd         => Self::BitAnd,
            TokenKind::BitOr          => Self::BitOr,
            TokenKind::BitXor         => Self::BitXor,
            TokenKind::BitNot         => Self::BitNot,
            TokenKind::BitShiftLeft   => Self::BitShiftLeft,
            TokenKind::BitShiftRight  => Self::BitShiftRight,

            //Litops
            TokenKind::LitTilde => Self::LitTilde,
            TokenKind::LitBang => Self::LitBang,
            TokenKind::LitAt => Self::LitAt,
            TokenKind::LitOctothorpe => Self::LitOctothorpe,
            TokenKind::LitDollarSign => Self::LitDollarSign,
            TokenKind::LitPercent => Self::LitPercent,
            TokenKind::LitCarrot => Self::LitCarrot,
            TokenKind::LitAmpersand => Self::LitAmpersand,
            TokenKind::LitStar => Self::LitStar,
            TokenKind::LitMinus => Self::LitMinus,
            TokenKind::LitEqual => Self::LitEqual,
            TokenKind::LitPlus => Self::LitPlus,
            TokenKind::LitUnderscore => Self::LitUnderscore,
            TokenKind::LitColon => Self::LitColon,
            TokenKind::LitDot => Self::LitDot,
            TokenKind::LitSlash=> Self::LitSlash,

            // Trigraphs
            TokenKind::DSLDelimiter   => Self::DSLDelimiter,
            TokenKind::Bottom         => Self::Bottom,
            TokenKind::SectionMarker  => Self::SectionMarker,

            TokenKind::Error          => Self::Error,
        }
    }
}

pub type SyntaxNode = rowan::SyntaxNode<VADA>;
pub type SyntaxElement = rowan::SyntaxElement<VADA>;
pub type SyntaxToken = rowan::SyntaxToken<VADA>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, FromPrimitive, ToPrimitive)]
pub enum VADA {}

impl rowan::Language for VADA {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        Self::Kind::from_u16(raw.0).unwrap()
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}

// Should I setup some tests here?