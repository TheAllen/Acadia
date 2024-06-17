#[macro_export]
macro_rules! function_string{
    ($func: ident) => {{
        stringify!($func)
    }};
}