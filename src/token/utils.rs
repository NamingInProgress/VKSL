#[macro_export]
macro_rules! T {
    ('(') => {
        $crate::token::TokenType::LParen
    };
    (')') => {
        $crate::token::TokenType::RParen
    };
    ('{') => {
        $crate::token::TokenType::LBrace
    };
    ('}') => {
        $crate::token::TokenType::RBrace
    };
    ('[') => {
        $crate::token::TokenType::LBracket
    };
    (']') => {
        $crate::token::TokenType::RBracket
    };
    (,) => {
        $crate::token::TokenType::Comma
    };
    (;) => {
        $crate::token::TokenType::Semi
    };
    (:) => {
        $crate::token::TokenType::Colon
    };
    (?) => {
        $crate::token::TokenType::Question
    };

    (+=) => {
        $crate::token::TokenType::OperatorAssign($crate::token::Operator::Plus)
    };
    (-=) => {
        $crate::token::TokenType::OperatorAssign($crate::token::Operator::Minus)
    };
    (*=) => {
        $crate::token::TokenType::OperatorAssign($crate::token::Operator::Mul)
    };
    (/=) => {
        $crate::token::TokenType::OperatorAssign($crate::token::Operator::Div)
    };
    (%=) => {
        $crate::token::TokenType::OperatorAssign($crate::token::Operator::Modulo)
    };
    (&=) => {
        $crate::token::TokenType::OperatorAssign($crate::token::Operator::BitAnd)
    };
    (|=) => {
        $crate::token::TokenType::OperatorAssign($crate::token::Operator::BitOr)
    };
    (^=) => {
        $crate::token::TokenType::OperatorAssign($crate::token::Operator::BitXor)
    };
    (<<=) => {
        $crate::token::TokenType::OperatorAssign($crate::token::Operator::Lsh)
    };
    (>>=) => {
        $crate::token::TokenType::OperatorAssign($crate::token::Operator::Rsh)
    };
    (>>>=) => {
        $crate::token::TokenType::OperatorAssign($crate::token::Operator::LogicalRsh)
    };

    (<-) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Merge)
    };
    (->) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Merge)
    };
    (++) => {
        $crate::token::TokenType::Operator($crate::token::Operator::PlusPlus)
    };
    (--) => {
        $crate::token::TokenType::Operator($crate::token::Operator::MinusMinus)
    };
    (&&) => {
        $crate::token::TokenType::Operator($crate::token::Operator::And)
    };
    (||) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Or)
    };
    (==) => {
        $crate::token::TokenType::Operator($crate::token::Operator::EqEq)
    };
    (!=) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Neq)
    };
    (>=) => {
        $crate::token::TokenType::Operator($crate::token::Operator::GreaterEq)
    };
    (<=) => {
        $crate::token::TokenType::Operator($crate::token::Operator::LessEq)
    };
    (<<) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Lsh)
    };
    (>>) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Rsh)
    };
    (>>>) => {
        $crate::token::TokenType::Operator($crate::token::Operator::LogicalRsh)
    };

    (=) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Assign)
    };
    (+) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Plus)
    };
    (-) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Minus)
    };
    (*) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Mul)
    };
    (/) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Div)
    };
    (.) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Dot)
    };
    (%) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Modulo)
    };
    (|) => {
        $crate::token::TokenType::Operator($crate::token::Operator::BitOr)
    };
    (&) => {
        $crate::token::TokenType::Operator($crate::token::Operator::BitAnd)
    };
    (^) => {
        $crate::token::TokenType::Operator($crate::token::Operator::BitXor)
    };
    (~) => {
        $crate::token::TokenType::Operator($crate::token::Operator::BitNegate)
    };
    (>) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Greater)
    };
    (<) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Less)
    };
    (!) => {
        $crate::token::TokenType::Operator($crate::token::Operator::Not)
    };

    (fn) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Fn)
    };
    (struct) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Struct)
    };
    (if) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::If)
    };
    (else) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Else)
    };
    (while) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::While)
    };
    (for) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::For)
    };
    (return) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Return)
    };
    (let) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Let)
    };
    (include) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Include)
    };
    (extension) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Extension)
    };
    (enable) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Enable)
    };
    (require) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Require)
    };
    (warn) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Warn)
    };
    (disable) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Disable)
    };
    (input) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Input)
    };
    (output) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Output)
    };
    (provide) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Provide)
    };
    (push_constants) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::PushConstants)
    };
    (uniform) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Uniform)
    };
    (buffer) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Buffer)
    };
    (std430) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::STD430)
    };
    (std140) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::STD140)
    };
    (readonly) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Readonly)
    };
    (writeonly) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Writeonly)
    };
    (break) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Break)
    };
    (continue) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Continue)
    };
    (flat) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Flat)
    };
    (smooth) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Smooth)
    };
    (noperspective) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Noperspective)
    };
    (yield) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Yield)
    };
    (nonuniform) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Nonuniform)
    };
    (const) => {
        $crate::token::TokenType::Keyword($crate::token::Keyword::Const)
    };

    ($ident:ident) => {
        $crate::token::TokenType::Ident(stringify!($ident).to_string())
    };

    ($ident:literal) => {
        $crate::token::TokenType::Ident($ident.to_string())
    };
}

#[macro_export]
macro_rules! Te {
    [] => {
        vec![]
    };

    [@munch $v:ident;] => {};

    [@munch $v:ident; <- $($tail:tt)*] => {
        $v.push($crate::parser::err::TokenExpectation::Exact($crate::T!(<-)));
        $crate::Te!(@munch $v; $($tail)*);
    };

    [@munch $v:ident; -> $($tail:tt)*] => {
        $v.push($crate::parser::err::TokenExpectation::Exact($crate::T!(->)));
        $crate::Te!(@munch $v; $($tail)*);
    };

    [@munch $v:ident; ++ $($tail:tt)*] => {
        $v.push($crate::parser::err::TokenExpectation::Exact($crate::T!(++)));
        $crate::Te!(@munch $v; $($tail)*);
    };

    [@munch $v:ident; -- $($tail:tt)*] => {
        $v.push($crate::parser::err::TokenExpectation::Exact($crate::T!(--)));
        $crate::Te!(@munch $v; $($tail)*);
    };

    [@munch $v:ident; ID $($tail:tt)*] => {
        $v.push($crate::parser::err::TokenExpectation::Ident);
        $crate::Te!(@munch $v; $($tail)*);
    };

    [@munch $v:ident; $head:tt $($tail:tt)*] => {
        $v.push($crate::parser::err::TokenExpectation::Exact($crate::T!($head)));
        $crate::Te!(@munch $v; $($tail)*);
    };

    [$($tok:tt)+] => {{
        let mut v = Vec::new();
        $crate::Te!(@munch v; $($tok)+);
        v
    }};
}
