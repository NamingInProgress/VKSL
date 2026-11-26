use mvutils::utils::UnwrapDisplay;
use crate::compiler::{VkslCompiler, VkslCompilerOptions};

pub mod parser;
pub mod token;
pub mod scope;
pub mod compiler;

fn main() {
    let compiler = VkslCompiler::new(VkslCompilerOptions {
        input: "simple.vksl".into(),
        output: "out.spv".into(),
        include: vec!["src".into(), "shaders".into()],
    });

    compiler.compile().unwrapd();
}