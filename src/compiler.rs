use crate::parser;
use crate::parser::Parser;
use crate::parser::ast::{IncludeFlavour, IncludeStmt};
use crate::parser::err::{ParseErr, ParseErrType};
use crate::scope::{const_eval, name_res, struct_res};
use crate::token::TokCtx;
use crate::token::tokenizer::Tokenizer;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[macro_export]
macro_rules! critical_error {
    () => {
        panic!("critical error: {}:{}", file!(), line!())
    };
}

pub struct VkslCompilerOptions {
    pub input: PathBuf,
    pub output: PathBuf,
    pub include: Vec<PathBuf>,
}

pub struct VkslCompiler {
    pub options: VkslCompilerOptions,
    pub included: HashSet<PathBuf>,
    pub queue: VecDeque<PathBuf>
}

impl VkslCompiler {
    pub fn new(options: VkslCompilerOptions) -> Self {
        Self {
            options,
            included: HashSet::new(),
            queue: VecDeque::new(),
        }
    }

    pub fn compile(mut self) -> parser::Result<()> {
        let input = std::path::absolute(self.options.input.clone()).map_err(Self::io_err)?;
        let mut result = self.process_file(input)?;

        while !self.queue.is_empty() {
            let next = self.queue.pop_front().unwrap();
            result.append(&mut self.process_file(next)?);
        }

        let global_scope = struct_res::handle_ast(&mut result)?;
        let result = name_res::conv_ast(result, global_scope.clone())?;
        let const_evaled = const_eval::collapse_constants(result, &global_scope)?;
        for stmt in const_evaled {
            println!("{stmt:#?}");
        }

        println!("=======");
        let g = global_scope.borrow();
        println!("{:#?}", g.symbols);

        Ok(())
    }

    fn process_file(&mut self, path: PathBuf) -> parser::Result<parser::ast::Ast> {
        let mut buffer = String::new();
        let parent_dir = path.parent().expect("Path is a file thus has a parent").to_path_buf();
        File::open(path).map_err(Self::io_err)?.read_to_string(&mut buffer).map_err(Self::io_err)?;
        let tokens = Tokenizer::new(buffer.chars(), Some(Path::new("simple.vksl").into()));
        let parser = Parser::new(tokens);
        let (result, includes) = parser.parse()?;

        for include in &includes {
            println!("{:?}", include);
        }

        self.process_includes(includes, parent_dir)?;

        Ok(result)
    }

    fn process_includes(&mut self, includes: Vec<IncludeStmt>, parent: PathBuf) -> parser::Result<()> {
        for include in includes {
            match include.flavour {
                IncludeFlavour::Local => {
                    let mut path = parent.clone();
                    path.push(include.path.clone());
                    let absolute_path = std::path::absolute(path).map_err(Self::io_err)?;
                    if !absolute_path.is_file() {
                        return Err(ParseErr {
                            ty: ParseErrType::IncludeNotFound(include.path),
                            ctx: include.path_tkn,
                            hint: None,
                        })
                    }

                    // hopefully means that symlinks to the same file arent included twice despite having different paths
                    let canonical_path = absolute_path.canonicalize().map_err(Self::io_err)?;

                    if self.included.insert(canonical_path) {
                        self.queue.push_back(absolute_path);
                    }
                }
                IncludeFlavour::Includes => {
                    let mut found = false;

                    for include_dir in &self.options.include {
                        let mut path = include_dir.clone();
                        path.push(include.path.clone());
                        let absolute_path = std::path::absolute(path).map_err(Self::io_err)?;
                        if !absolute_path.is_file() {
                            continue;
                        }

                        found = true;
                        let canonical_path = absolute_path.canonicalize().map_err(Self::io_err)?;

                        if self.included.insert(canonical_path) {
                            self.queue.push_back(absolute_path);
                        }
                        break;
                    }

                    if !found {
                        return Err(ParseErr {
                            ty: ParseErrType::IncludeNotFound(include.path),
                            ctx: include.path_tkn,
                            hint: None,
                        })
                    }
                }
            }
        }
        Ok(())
    }

    fn io_err(e: std::io::Error) -> ParseErr {
        ParseErr {
            ty: ParseErrType::IoError(Arc::new(e)),
            ctx: TokCtx::empty(),
            hint: None,
        }
    }
}
