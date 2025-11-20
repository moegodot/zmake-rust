use crate::make_builtin;

make_builtin! {
    self => {
        "moe.kawayi:zmake@1.0.0" => {
            ToolType =>
            {
                COMPILER => "c/compiler",
                PREPROCESSOR => "c/preprocessor"
            },
            Tool =>{
                GCC => "c/gcc",
                CLANG => "c/clang",
                MSVC => "c/msvc"
            }
        }
    }
}
