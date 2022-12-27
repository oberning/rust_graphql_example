macro_rules! bool_lambda {
    ($a:expr, $type:ty, $b:expr) => {
        match $a {
            $b => |_, _| true,
            _ => |x, y| x == y,
        }
    };
}