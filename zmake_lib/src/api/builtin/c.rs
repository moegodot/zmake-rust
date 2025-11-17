use crate::make_builtin;
use ahash::AHashMap;
use static_init::dynamic;

#[dynamic(lazy)]
pub static BUILTINS: AHashMap<String, crate::api::id::Id> = make_builtin! {
    "moe.kawayi:zmake@1.0.0" => {
        crate::api::id::IdType::ToolType =>
        {
            "compiler" => "c.compiler",
            "preprocessor" => "c.preprocessor"
        },
        crate::api::id::IdType::ToolName =>{
            "gcc" => "c.gcc",
            "clang" => "c.clang",
            "msvc" => "c.msvc"
        }
    }
};
