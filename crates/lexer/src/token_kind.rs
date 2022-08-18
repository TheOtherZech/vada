use logos::Logos;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Logos, Ord, PartialOrd, Eq)]
pub enum TokenKind {
    #[regex("[ \n]+")]
    Whitespace,

    // TODO: Unicode
    #[regex("[A-Za-z_][A-Za-z0-9_]*")]
    Ident,

    #[token("_")]
    Underscore,   // The implicit

    // START LITERALS

    #[regex("0b[0-1]+")]
    BinaryLiteral,

    #[regex("0o[0-7]+")]
    OctalLiteral,

    #[regex("0x[0-9A-F]+")]
    HexLiteral,

    #[regex(r"[1-9][0-9]*v[0-9]+")]
    AddressLiteral,

    // TODO: This might not play well with binding the implicit immediately following an int-bound statement
    #[regex(r"[0-9][0-9_]*")]
    IntLiteral,

    #[regex(r"[0-9]+[0-9_]*d[0-9]+[0-9_]")]
    RandIntLiteral,

    #[regex(r"[0-9][0-9_]*\.[0-9][0-9_]*")]
    FloatLiteral,

    #[regex(r"[0-9]+\\.[0-9_]*d[0-9]+\\.[0-9_]*")]
    RandFloatLiteral,

    // Strings-as-tokens is a bad idea, as it doesn't allow interior lexing.
    #[regex(r#""[A-Za-z0-9 _]*""#)]
    String,

    // END LITERALS

    // START PREFIXES

    #[token("!")]
    Bang,

    #[token("@")]
    At,

    #[token("#")]
    Octothorpe,

    #[token("$")]
    DollarSign,

    // END PREFIXES

    // START OPERATORS

    #[token("%")]
    Percent,

    #[token("^")]
    Carrot,

    #[token("&")]
    Ampersand,

    #[token("*")]
    Star,

    #[token("-")]
    Minus,

    #[token("+")]
    Plus,

    #[token("=")]
    Equals,

    #[token("|")]
    Bar,

    // Range Operator
    #[token(r"..")]
    DotDot,

    #[token(".")]
    Dot,

    #[token("<")]
    LAngleBrack,

    #[token(">")]
    RAngleBrack,

    #[token("/")]
    Slash,

    #[token("?")]
    QMark,

    #[token("~")]
    Tilde,

    // Accessor Digraph
    #[token("::")]
    ColonColon,

    // Comparison Digraphs
    #[token("!=")]
    NotEqual,

    #[token("<=")]
    LessEqual,

    #[token(">=")]
    GreaterEqual,

    #[token("~=")]
    PatternEqual,

    #[token("!~")]
    NotPattern,

    // Assignment Digraphs
    #[token(":=")]
    Walrus,
    #[token(":?")]
    Mustache,

    // Arrow Digraphs
    #[token("->")]
    RArrow,

    #[token("<~")]
    LSquiggleArrow,
    #[token("~>")]
    RSquiggleArrow,

    // Bitwise Operator Digraphs
    #[token(".&")]
    BitAnd,
    #[token(".|")]
    BitOr,
    #[token(".^")]
    BitXor,
    #[token(".~")]
    BitNot,
    #[token(".<")]
    BitShiftLeft,
    #[token(".>")]
    BitShiftRight,

    // END OPERATORS

    // START ENCLOSING

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBrack,

    #[token("]")]
    RBrack,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("`")]
    Backtick,

    #[token("\'")]
    SingleQuote,

    #[token("\"")]
    DoubleQuote,

    #[token("<;")]
    LBird,
    #[token(";>")]
    RBird,

    #[token("<^")]
    LJuke,
    #[token("^>")]
    RJuke,

    #[token("[|")]
    LDee,
    #[token("|]")]
    RDee,

    // END ENCLOSING

    #[token("\\")]
    Backslash,

    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("♢")]
    Diamond,

    #[regex(r#"//.*"#)]
    Comment,

    // START LITOPS

    #[token("{~}")]
    LitTilde,

    #[token("{!}")]
    LitBang,

    #[token("{@}")]
    LitAt,

    #[token("{#}")]
    LitOctothorpe,

    #[token("{$}")]
    LitDollarSign,

    #[token("{%}")]
    LitPercent,

    #[token("{^}")]
    LitCarrot,

    #[token("{&}")]
    LitAmpersand,

    #[token("{*}")]
    LitStar,

    #[token("{-}")]
    LitMinus,

    #[token("{=}")]
    LitEqual,

    #[token("{+}")]
    LitPlus,

    #[token("{_}")]
    LitUnderscore,

    #[token("{:}")]
    LitColon,

    #[token("{.}")]
    LitDot,

    #[token("{/}")]
    LitSlash,

    // END LITOPS

    // Core Trigraphs Start Here
    #[token("^.^")]
    DSLDelimiter,

    #[token("_|_")]
    Bottom,

    #[token("---")]
    SectionMarker,

    #[token("...")]
    DotDotDot,

    // The catch-all
    #[error]
    Error,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace | Self::Comment)
    }

    pub fn is_literal(self) -> bool {
        return self >= TokenKind::BinaryLiteral 
            && self < TokenKind::String;
    }

    pub fn is_prefix(self) -> bool {
        return matches!(self, 
            Self::Bang 
            | Self::At 
            | Self::Octothorpe
            | Self::DollarSign
        );
    }

    pub fn is_op(self) -> bool {
        return self >= Self::Percent
            && self <= Self::BitShiftRight;
    }

    pub fn is_enclosing(self) -> bool {
        return self >= Self::LParen
            && self <= Self::RDee;
    }

    pub fn is_litop(self) -> bool {
        return self >= Self::LitTilde
            && self <= Self::LitSlash
    }

    // TODO: Build out more functions to classify tokens into subsets.
    //       Maybe create a meta-enum for token sets?
}


impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Whitespace => "whitespace",
            Self::Ident => "identifier",

            // Self::Number => "number",
            Self::BinaryLiteral    => "binary number",
            Self::OctalLiteral     => "octal number",
            Self::HexLiteral       => "hex number",
            Self::AddressLiteral   => "memory address",
            Self::IntLiteral       => "integer",
            Self::RandIntLiteral   => "random integer",
            Self::FloatLiteral     => "float",
            Self::RandFloatLiteral => "random float",

            Self::String => "string",

            Self::Backtick   => "`",
            Self::Tilde      => "~",
            Self::Bang       => "!",
            Self::At         => "@",
            Self::Octothorpe => "#",
            Self::DollarSign => "$",
            Self::Percent    => "%",
            Self::Carrot     => "^",
            Self::Ampersand  => "&",
            Self::Star       => "*",
            Self::LParen     => "(",
            Self::RParen     => ")",
            Self::Minus      => "-",
            Self::Underscore => "_",
            Self::Equals     => "=",
            Self::Plus       => "+",
            Self::LBrack     => "[",
            Self::RBrack     => "]",
            Self::LBrace     => "{",
            Self::RBrace     => "}",            
            Self::Backslash  => "\\",
            Self::Bar        => "|",
            Self::Semicolon  => ";",
            Self::Colon      => ":",
            Self::SingleQuote => "'",
            Self::DoubleQuote => "\"",
            Self::Comma       => ",",
            Self::Dot         => ".",
            Self::LAngleBrack => "<",
            Self::RAngleBrack => ">",
            Self::Slash       => "/",
            Self::QMark       => "?",
            Self::Diamond     => "♢",
            Self::Comment     => "comment",

            // Digraphs
            Self::ColonColon   => "::",

            Self::NotEqual     => "!=",
            Self::LessEqual    => "<=",
            Self::GreaterEqual => ">=",
            Self::PatternEqual => "~=",
            Self::NotPattern   => "!~",

            // Assignment Digraphs
            Self::Walrus   => ":=",
            Self::Mustache => ":?",

            // Arrow Digraphs
            Self::RArrow         => "->",
            Self::LSquiggleArrow => "<~",
            Self::RSquiggleArrow => "~>",
            Self::LBird          => "<;",
            Self::RBird          => ";>",

            Self::DotDot => "..",
            Self::DotDotDot => "...",

            Self::LJuke => "<^",
            Self::RJuke => "^>",
            Self::LDee => "[|",
            Self::RDee => "|]",

            Self::BitAnd => ".&",
            Self::BitOr  => ".|",
            Self::BitXor => ".^",
            Self::BitNot => ".~",
            Self::BitShiftLeft => ".<",
            Self::BitShiftRight => ".>",

            //LitOPs
            Self::LitTilde      => "{~}",
            Self::LitBang       => "{!}",
            Self::LitAt         => "{@}",
            Self::LitOctothorpe => "{#}",
            Self::LitDollarSign => "{$}",
            Self::LitPercent    => "{%}",
            Self::LitCarrot     => "{^}",
            Self::LitAmpersand  => "{&}",
            Self::LitStar       => "{*}",
            Self::LitMinus      => "{-}",
            Self::LitEqual      => "{=}",
            Self::LitPlus       => "{+}",
            Self::LitUnderscore => "{_}",
            Self::LitColon      => "{:}",
            Self::LitDot        => "{.}",
            Self::LitSlash      => "{/}",

            // Trigraphs
            Self::DSLDelimiter => "^.^",
            Self::Bottom => "_|_",
            Self::SectionMarker => "---",
            Self::Error => "an unrecognized token",

            //_ => "Ghost"
        })
    }
}
/*
All Possible Pairs:
 
    !  @  #  $  %  ^  &  *  ~  +  -  =  ?  /  <  >  .  ,  :

!   !! !@ !# !$ !% !^ !& !* !~ !+ !- != !? !/ !< !> !. !, !:
@   @! @@ @# @$ @% @^ @& @* @~ @+ @- @= @? @/ @< @> @. @, @:
#   #! #@ ## #$ #% #^ #& #* #~ #+ #- #= #? #/ #< #> #. #, #:
$   $! $@ $# $$ $% $^ $& $* $~ $+ $- $= $? $/ $< $> $. $, $:
%   %! %@ %# %$ %% %^ %& %* %~ %+ %- %= %? %/ %< %> %. %, %:
^   ^! ^@ ^# ^$ ^% ^^ ^& ^* ^~ ^+ ^- ^= ^? ^/ ^< ^> ^. ^, ^:
&   &! &@ &# &$ &% &^ && &* &~ &+ &- &= &? &/ &< &> &. &, &:
*   *! *@ *# *$ *% *^ *& ** *~ *+ *- *= *? __ *< *> *. *, *:
~   ~! ~@ ~# ~$ ~% ~^ ~& ~* ~~ ~+ ~- ~= ~? ~/ ~< ~> ~. ~, ~:
+   +! +@ +# +$ +% +^ +& +* +~ ++ +- += +? +/ +< +> +. +, +:
-   -! -@ -# -$ -% -^ -& -* -~ -+ -- -= -? -/ -< -> -. -, -:
=   =! =@ =# =$ =% =^ =& =* =~ =+ =- == =? =/ =< => =. =, =:
?   ?! ?@ ?# ?$ ?% ?^ ?& ?* ?~ ?+ ?- ?= ?? ?/ ?< ?> ?. ?, ?:
/   /! /@ /# /$ /% /^ /& _  /~ /+ /- /= /? // /< /> /. /, /:
<   <! <@ <# <$ <% <^ <& <* <~ <+ <- <= <? </ << <> <. <, <:
>   >! >@ ># >$ >% >^ >& >* >~ >+ >- >= >? >/ >< >> >. >, >:
.   .! .@ .# .$ .% .^ .& .* .~ .+ .- .= .? ./ .< .> .. ., .:
,   ,! ,@ ,# ,$ ,% ,^ ,& ,* ,~ ,+ ,- ,= ,? ,/ ,< ,> ,. ,, ,:
:   :! :@ :# :$ :% :^ :& :* :~ :+ :- := :? :/ :< :> :. :, ::

Trigraphs
<+>
<->
<_>
<^>
<&>

Set Operations

|+| Union
|-| Difference
|^| Intersection
|*| Cartesian Product

Set Equality
?

Array/List Comprehension Stuff
... Spread/Fill (prefix operator?)

*/


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lexer;

    fn check(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);

        let token = lexer.next().unwrap();
        assert_eq!(token.kind, kind);
        assert_eq!(token.text, input);
    }

    fn check_seq(input: &str, expect: Vec<TokenKind>) {
        let mut lexer = Lexer::new(input);
        for t in expect.into_iter() {
            let token = lexer.next().unwrap();
            println!("{:?} {:?} | {:?}", token.text , token.kind, t);
            assert_eq!(token.kind, t);
        }
    }

    #[test]
    fn lex_spaces_and_newlines() {
        check("  \n ", TokenKind::Whitespace);
    }

    #[test]
    fn check_ord() {
        assert!(TokenKind::DotDotDot > TokenKind::Ident);
    }


    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", TokenKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("ab123cde456", TokenKind::Ident);
    }

    #[test]
    fn lex_mixed_case_identifier() {
        check("ABCdef", TokenKind::Ident);
    }

    #[test]
    fn lex_identifier_with_leading_underscore(){
        check("_abcdef", TokenKind::Ident);
    }

    #[test]
    fn lex_identifier_with_middle_underscore(){
        check("abc_def", TokenKind::Ident);
    }

    #[test]
    fn lex_identifier_with_trailing_underscore(){
        check("abcdef_", TokenKind::Ident);
    }

    #[test]
    fn lex_single_char_identifier() {
        check("x", TokenKind::Ident);
    }

    #[test]
    fn lex_int() {
        check("123456", TokenKind::IntLiteral);
    }

    #[test]
    fn lex_float() {
        check("12.3456", TokenKind::FloatLiteral);
    }

    #[test]
    fn lex_plus() {
        check("+", TokenKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", TokenKind::Minus);
    }

    #[test]
    fn lex_star() {
        check("*", TokenKind::Star);
    }

    #[test]
    fn lex_slash() {
        check("/", TokenKind::Slash);
    }

    #[test]
    fn lex_equals() {
        check("=", TokenKind::Equals);
    }

    #[test]
    fn lex_left_parenthesis() {
        check("(", TokenKind::LParen);
    }

    #[test]
    fn lex_right_parenthesis() {
        check(")", TokenKind::RParen);
    }

    #[test]
    fn lex_left_brace() {
        check("{", TokenKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check("}", TokenKind::RBrace);
    }

    #[test]
    fn lex_dot_dot() {
        check("..", TokenKind::DotDot);
    }

    #[test]
    fn lex_dot_dot_dot() {
        check("...", TokenKind::DotDotDot);
    }

    #[test]
    fn lex_ident_range() {
        check_seq(
            "A..B",
            vec![
                TokenKind::Ident,
                TokenKind::DotDot,
                TokenKind::Ident
            ]
        )
    }

    #[test]
    fn lex_int_range() {
        check_seq(
            r"0..1",
            vec![
                TokenKind::IntLiteral,
                TokenKind::DotDot,
                TokenKind::IntLiteral
            ]
        )
    }

    #[test]
    fn lex_float_range() {
        check_seq(
            "0.0..1.0",
            vec![
                TokenKind::FloatLiteral,
                TokenKind::DotDot,
                TokenKind::FloatLiteral
            ]
        )
    }

    #[test]
    fn lex_binary_range() {
        check_seq(
            "0b001..0b111",
            vec![
                TokenKind::BinaryLiteral,
                TokenKind::DotDot,
                TokenKind::BinaryLiteral
            ]
        )
    }

    #[test]
    fn lex_sanity_test() {
        check_seq("4.", vec![
            TokenKind::IntLiteral,
            TokenKind::Dot,
        ])
    }

    #[test]
    fn lex_comment() {
        check(r#"// foo"#, TokenKind::Comment);
    }

}