macro_rules! bool_lambda {
    ($a:ident, "integer") => {
        match $a {
            0 => |_, _| true,
            _ => |x, y| x == y,
        }
    };
    ($a:ident, "string") => {
        match $a.as_str() {
            "" => |_, _| true,
            _ => |x, y| x == y,
        }
    };
}