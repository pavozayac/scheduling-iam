use time::UtcDateTime;

pub fn utc_to_primitive_datetime(utc: UtcDateTime) -> time::PrimitiveDateTime {
    time::PrimitiveDateTime::new(utc.date(), utc.time())
}
