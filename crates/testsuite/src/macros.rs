#[doc(hidden)]
#[macro_export]
macro_rules! _function_name {
    () => {{
        fn f() {}
        fn type_name_of_val<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let mut name = type_name_of_val(f).strip_suffix("::f").unwrap_or("");
        while let Some(rest) = name.strip_suffix("::{{closure}}") {
            name = rest;
        }
        name
    }};
}

#[macro_export]
macro_rules! register {
    () => {
        $crate::_macro_support::Probe::register($crate::_function_name!());
    };
}

#[macro_export]
macro_rules! probe {
    ($target:expr) => {{
        $crate::_macro_support::Probe::probe(::std::format!("{:#?}", $target));
        $target
    }};
}

#[macro_export]
macro_rules! footprints {
    () => {
        $crate::_macro_support::Probe::footprints($crate::_function_name!())
    };
}

#[macro_export]
macro_rules! unittest {
    ($name:ident, $closure:expr) => {
        #[test]
        fn $name() {
            $crate::_macro_support::source_exec(
                concat!(stringify!($name), ".hs"),
                ::std::module_path!(),
                ::std::env!("CARGO_MANIFEST_DIR"),
                $closure,
            );
        }
    };

    ($pin_to:expr, $name:ident, $closure:expr) => {
        #[test]
        fn $name() {
            $crate::_macro_support::source_exec(
                concat!(stringify!($name), ".hs"),
                $pin_to,
                ::std::env!("CARGO_MANIFEST_DIR"),
                $closure,
            );
        }
    };
}
