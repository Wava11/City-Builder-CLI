#[macro_export]
macro_rules! tick {
    ( $timer:expr, $time:expr ) => {
        if !$timer.0.tick($time.delta()).just_finished() {
            return;
        }
    };
}

#[macro_export]
macro_rules! foo {
    ( $name:ident{$($field_name:ident: $field_type:ty),*} ) => {
        #[derive(Bundle)]
        struct concat_idents!($name,Bundle) {
            $(
                $field_name: $field_type,
            )*
        }
    };
}

