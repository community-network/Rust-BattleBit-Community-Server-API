use num_derive::FromPrimitive;

#[derive(FromPrimitive, Debug)]
pub enum MapDayNight {
    Day = 0,
    Night = 1,
}
