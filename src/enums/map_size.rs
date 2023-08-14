use num_derive::FromPrimitive;

#[derive(FromPrimitive, Debug)]
pub enum MapSize {
    None = 0,
    _8v8 = 8,
    _16vs16 = 16,
    _32vs32 = 32,
    _64vs64 = 64,
    _127vs127 = 90,
}
