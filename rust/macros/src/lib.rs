#[macro_export]
macro_rules! hashmap {
    ()=>{
        ::std::collections::HashMap::new()
    };
    ( $( $x:expr => $y:expr ),* ) => {{
        let mut mp = ::std::collections::HashMap::new();
        $(
            mp.insert($x, $y);
        )*
        mp
    }};
    ( $( $x:expr => $y:expr ),+ $(,)? ) => {{
        let mut mp = ::std::collections::HashMap::new();
        $(
            mp.insert($x, $y);
        )*
        mp
    }};
}
