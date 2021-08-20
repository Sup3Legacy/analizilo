#[macro_export]
macro_rules! fixe_parsing {
    ($e:ty, [ $( ($x:ident, $y:expr) ),* ]) => {
        impl $e {
            pub fn from_str(arg: &str) -> Self {
                match arg {
                    $(
                        $y => <$e>::$x,
                    )*
                    _ => panic!("Could not parse from &str."),
                }
            }
        }
    };
}
