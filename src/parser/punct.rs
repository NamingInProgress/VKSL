use std::marker::PhantomData;
use crate::parser;
use crate::parser::err::{ParseErrType};
use crate::parser::Parser;
use crate::token::{TokCtx, Token, TokenType};

impl<I: Iterator<Item=char>> Parser<I> {
    #[allow(unused)]
    pub fn parse_punctuated<T, G>(&mut self, separator: TokenType, stop: TokenType, allow_trailing: bool, generator: G) -> Punctuated<'_, T, I, G>
    where
        G: for<'b> Fn(&'b mut Parser<I>) -> parser::Result<T>
    {
        Punctuated {
            _marker: Default::default(),
            generator,
            parser: self,
            separator,
            stop,
            allow_trailing,
            next_sep: None,
        }
    }
}

pub struct Punctuated<'a, T, I, G>
    where
        G: for<'b> Fn(&'b mut Parser<I>) -> parser::Result<T>,
        I: Iterator<Item=char> + 'a
{
    _marker: PhantomData<&'a ()>,
    generator: G,
    parser: &'a mut Parser<I>,
    separator: TokenType,
    stop: TokenType,
    allow_trailing: bool,
    next_sep: Option<TokCtx>
}

impl<'a, T, I, G> Iterator for Punctuated<'a, T, I, G>
where
    G: for<'b> Fn(&'b mut Parser<I>) -> parser::Result<T>,
    I: Iterator<Item=char> + 'a,
    T: Sized + 'static
{
    type Item = parser::Result<(T, Option<TokCtx>)>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.inner() {
            Ok(o) => o.map(Ok),
            Err(e) => Some(Err(e))
        }
    }
}

impl<'a, T, I, G> Punctuated<'a, T, I, G>
where
    G: for<'b> Fn(&'b mut Parser<I>) -> parser::Result<T>,
    I: Iterator<Item=char> + 'a,
    T: Sized + 'static
{
    fn inner(&mut self) -> parser::Result<Option<(T, Option<TokCtx>)>> {
        if let Some(Token { ty, .. }) = self.parser.peek_token()? && ty == self.stop {
            return Ok(None);
        }
        let t = (self.generator)(self.parser)?;
        if let Some(Token { ty, ctx }) = self.parser.peek_token()? && ty == self.separator {
            self.parser.unwrap_next()?;
            self.next_sep = Some(ctx);
        }
        if let Some(Token { ty, ctx }) = self.parser.peek_token()? && ty == self.stop {
            if !self.allow_trailing && self.next_sep.is_some() {
                let hint = format!("trailing `{:?}` found, but this is actually illegal in this context, consider removing the `{:?}`", self.separator, self.separator);
                let e = self.parser.token_err_with_hint(ParseErrType::IllegalTrailingSeparator, ctx, hint);
                return Err(e);
            }
        }
        Ok(Some((t, self.next_sep.take())))
    }

    pub fn try_collect(mut self) -> parser::Result<Vec<(T, Option<TokCtx>)>> {
        let mut result = Vec::new();
        while let Some(next) = self.next() {
            result.push(next?);
        }
        Ok(result)
    }
}