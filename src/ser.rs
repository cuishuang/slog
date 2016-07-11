use std::io;
use serialize::hex::ToHex;

use super::logger::RecordInfo;

/// Value that can be serialized
///
/// Loggers require values in key-value pairs to
/// implement this trait.
///
pub trait Serialize: Send + Sync + 'static {
    /// Serialize self into `Serializer`
    ///
    /// Structs implementing this trait should generally
    /// only call respective methods of `serializer`.
    fn serialize(&self, rinfo: &RecordInfo, key: &str, serializer: &mut Serializer);
}

/// Value that can be serialized and stored
/// in logger itself.
///
/// As Loggers itself must be thread-safe, they can only
/// store values implementing this trait.
pub trait SyncSerialize: Send + Sync + 'static + Serialize {}


/// Serializer
///
/// Drains using `Format` will internally use
/// types implementing this trait.
pub trait Serializer {
    /// Emit bool
    fn emit_bool(&mut self, key: &str, val: bool);
    /// Emit `()`
    fn emit_unit(&mut self, key: &str);
    /// Emit `None`
    fn emit_none(&mut self, key: &str);
    /// Emit char
    fn emit_char(&mut self, key: &str, val: char);
    /// Emit bytes
    fn emit_bytes(&mut self, key: &str, val: &[u8]);
    /// Emit u8
    fn emit_u8(&mut self, key: &str, val: u8);
    /// Emit i8
    fn emit_i8(&mut self, key: &str, val: i8);
    /// Emit u16
    fn emit_u16(&mut self, key: &str, val: u16);
    /// Emit i16
    fn emit_i16(&mut self, key: &str, val: i16);
    /// Emit u32
    fn emit_u32(&mut self, key: &str, val: u32);
    /// Emit i32
    fn emit_i32(&mut self, key: &str, val: i32);
    /// Emit f32
    fn emit_f32(&mut self, key: &str, val: f32);
    /// Emit u64
    fn emit_u64(&mut self, key: &str, val: u64);
    /// Emit i64
    fn emit_i64(&mut self, key: &str, val: i64);
    /// Emit f64
    fn emit_f64(&mut self, key: &str, val: f64);
    /// Emit usize
    fn emit_usize(&mut self, key: &str, val: usize);
    /// Emit isize
    fn emit_isize(&mut self, key: &str, val: isize);
    /// Emit str
    fn emit_str(&mut self, key: &str, val: &str);
}

impl Serialize for str {
    fn serialize(&self, _rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) {
        serializer.emit_str(key, self)
    }
}

impl Serialize for [u8] {
    fn serialize(&self, _rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) {
        serializer.emit_bytes(key, self)
    }
}

impl SyncSerialize for [u8] {}

impl Serialize for Vec<u8> {
    fn serialize(&self, _rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) {
        serializer.emit_bytes(key, self.as_slice())
    }
}

impl SyncSerialize for Vec<u8> {}


impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) {
        match *self {
            Some(ref s) => s.serialize(rinfo, key, serializer),
            None => serializer.emit_none(key),
        }
    }
}


impl<T: Serialize> SyncSerialize for Option<T> {}

impl Serialize for String {
    fn serialize(&self, _rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) {
        serializer.emit_str(key, self.as_str())
    }
}

impl SyncSerialize for String {}


macro_rules! impl_serialize_for{
    ($t:ty, $f:ident) => {
        impl Serialize for $t {
            fn serialize(&self, _rinfo : &RecordInfo, key : &str, serializer : &mut Serializer) {
                serializer.$f(key, *self)
            }
        }

        impl SyncSerialize for $t where $t : Send+Sync { }
    };
}

impl_serialize_for!(&'static str, emit_str);
impl_serialize_for!(usize, emit_usize);
impl_serialize_for!(isize, emit_isize);
impl_serialize_for!(bool, emit_bool);
impl_serialize_for!(char, emit_char);
impl_serialize_for!(u8, emit_u8);
impl_serialize_for!(i8, emit_i8);
impl_serialize_for!(u16, emit_u16);
impl_serialize_for!(i16, emit_i16);
impl_serialize_for!(u32, emit_u32);
impl_serialize_for!(i32, emit_i32);
impl_serialize_for!(f32, emit_f32);
impl_serialize_for!(u64, emit_u64);
impl_serialize_for!(i64, emit_i64);
impl_serialize_for!(f64, emit_f64);

impl<S: Serialize, F: 'static + Sync + Send + Fn(&RecordInfo) -> S> Serialize for F {
    fn serialize(&self, rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) {
        (*self)(rinfo).serialize(rinfo, key, serializer)
    }
}

impl<S: Serialize, F: 'static + Sync + Send + Fn(&RecordInfo) -> S> SyncSerialize for F {}


impl<W : io::Write+?Sized> Serializer for W {
    fn emit_none(&mut self, key: &str) {
        write!(self, "{}: {}", key, "None").unwrap()
    }
    fn emit_unit(&mut self, key: &str) {
        write!(self, "{}: ()", key, ).unwrap()
    }

    fn emit_bool(&mut self, key: &str, val: bool) {
        write!(self, "{}: {}", key, val).unwrap()
    }

    fn emit_char(&mut self, key: &str, val: char) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_bytes(&mut self, key: &str, val: &[u8]) {
        write!(self, "{}: {}", key, val.to_hex()).unwrap()
    }

    fn emit_usize(&mut self, key: &str, val: usize) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_isize(&mut self, key: &str, val: isize) {
        write!(self, "{}: {}", key, val).unwrap()
    }

    fn emit_u8(&mut self, key: &str, val: u8) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_i8(&mut self, key: &str, val: i8) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_u16(&mut self, key: &str, val: u16) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_i16(&mut self, key: &str, val: i16) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_u32(&mut self, key: &str, val: u32) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_i32(&mut self, key: &str, val: i32) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_f32(&mut self, key: &str, val: f32) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_u64(&mut self, key: &str, val: u64) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_i64(&mut self, key: &str, val: i64) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_f64(&mut self, key: &str, val: f64) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_str(&mut self, key: &str, val: &str) {
        write!(self, "{}: {}", key, val).unwrap()
    }
}
