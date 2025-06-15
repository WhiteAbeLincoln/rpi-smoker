// serde only supports paths for defaults https://github.com/serde-rs/serde/issues/368#issuecomment-1627677992
macro_rules! gen_def {
    ($name:ident) => {
        paste::paste! {
            #[allow(dead_code)]
            pub fn [<def_ $name>]<const V: $name>() -> $name { V }
            #[allow(dead_code)]
            pub fn [<def_from_ $name>]<T: From<$name>, const V: $name>() -> T { V.into() }
        }
    };
}

gen_def!(u128);
gen_def!(u64);
gen_def!(u32);
gen_def!(u16);
gen_def!(u8);
gen_def!(i128);
gen_def!(i64);
gen_def!(i32);
gen_def!(i16);
gen_def!(i8);
gen_def!(bool);
