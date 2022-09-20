#[macro_export]
macro_rules! word {
    ($($tt:tt)*) => ($crate::Word([$($crate::Letter::$tt),*]))
}
