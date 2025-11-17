#[macro_export]
macro_rules! make_builtin {
    { $( $id:literal => { $($type:path => { $($key:literal => $value:literal),* }),* } ),* } => {
        {
            let mut map = ::ahash::AHashMap::<::std::string::String, $crate::api::id::Id>::new();

            $(
                let id = $id;

                $(
                    let id_type: &'static str = $type.into();

                    $(
                        let value = format!("{}#{}::{}", id,id_type,$value);
                        let value = <$crate::api::id::Id as ::std::str::FromStr>::from_str(&value).unwrap();

                        map.insert($key.to_string(),value);
                    )*
                )*
            )*

            map
        }
    }
}
