#[macro_export]
macro_rules! f {
    ( $x:expr, $( $y:ident ),* ) => {
        {
            $x.as_ref()$(
              .and_then(|f| f.$y.as_ref() )
            )*
        }
    };
}

#[macro_export]
macro_rules! m {
    ( $x:expr, $( $y:ident ),* ) => {
        {
            $x.as_ref()$(
              .map(|x| &x.$y )
            )*
        }
    };
}
