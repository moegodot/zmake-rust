use crate::make_builtin;
use ahash::AHashMap;
use static_init::dynamic;

#[dynamic(lazy)]
pub static BUILTINS: AHashMap<String, crate::api::id::Id> = make_builtin! {
    "moe.kawayi:zmake@1.0.0" => {
        crate::api::id::IdType::ToolType =>
        {
            "compiler" => "cpp.compiler",
            "preprocessor" => "cpp.preprocessor"
        },
        crate::api::id::IdType::ToolName =>{
            "gcc" => "cpp.gcc",
            "clang" => "cpp.clang",
            "msvc" => "cpp.msvc"
        }
    }
};
