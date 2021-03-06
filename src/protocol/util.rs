use std::io;
use std::io::prelude::*;

use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::UTF_16LE;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use ::{TdsResult};

#[doc(hidden)]
pub trait WriteCStr {
    fn write_cstr(&mut self, s: &str) -> io::Result<()>;
}

impl<W: Write> WriteCStr for W {
    fn write_cstr(&mut self, s: &str) -> io::Result<()> {
        try!(self.write_all(s.as_bytes()));
        Ok(try!(self.write_u8(0)))
    }
}

#[doc(hidden)]
pub trait WriteUtf16 {
    fn write_as_utf16(&mut self, s: &str) -> TdsResult<usize>;
}

impl<W: Write> WriteUtf16 for W {
    /// Writes a UTF-16 string with double null terminator
    fn write_as_utf16(&mut self, s: &str) -> TdsResult<usize> {
        let bytes = try!(UTF_16LE.encode(s, EncoderTrap::Strict));
        try!(self.write_all(&bytes));
        Ok(bytes.len())
    }
}


#[doc(hidden)]
pub trait ReadCharStream {
    fn read_varchar(&mut self, length: usize) -> TdsResult<String>;
    fn read_us_varchar(&mut self) -> TdsResult<String>;
    fn read_b_varchar(&mut self) -> TdsResult<String>;
}

pub trait WriteCharStream {
    fn write_varchar(&mut self, s: &str) -> TdsResult<Vec<u8>>;
    fn write_us_varchar(&mut self, s: &str) -> TdsResult<()>;
    fn write_b_varchar(&mut self, s: &str) -> TdsResult<()>;
}

impl<R: Read> ReadCharStream for R {
    fn read_varchar(&mut self, length: usize) -> TdsResult<String> {
        let length = length * 2;
        let mut bytes: Vec<u8> = vec![0; length];
        assert_eq!(try!(self.read(&mut bytes[..])), length);
        Ok(try!(UTF_16LE.decode(&bytes, DecoderTrap::Strict)))
    }

    #[inline]
    fn read_us_varchar(&mut self) -> TdsResult<String> {
        let len = try!(self.read_u16::<LittleEndian>()) as usize;
        self.read_varchar(len)
    }

    #[inline]
    fn read_b_varchar(&mut self) -> TdsResult<String> {
        let len = try!(self.read_u8()) as usize;
        self.read_varchar(len)
    }
}

impl<W: Write> WriteCharStream for W {
    fn write_varchar(&mut self, s: &str) -> TdsResult<Vec<u8>> {
        Ok(try!(UTF_16LE.encode(&s, EncoderTrap::Strict)))
    }

    fn write_us_varchar(&mut self, s: &str) -> TdsResult<()> {
        let bytes = try!(self.write_varchar(s));
        let len = s.len();
        assert!(len < 0xFFFF);
        try!(self.write_u16::<LittleEndian>(len as u16));
        try!(self.write_all(&bytes));
        Ok(())
    }

    fn write_b_varchar(&mut self, s: &str) -> TdsResult<()> {
        let bytes = try!(self.write_varchar(s));
        let len = s.len();
        assert!(len < 0xFF);
        try!(self.write_u8(len as u8));
        try!(self.write_all(&bytes));
        Ok(())
    }
}

pub trait FromPrimitive<T>: Sized {
    fn from(i: T) -> Option<Self>;
}

macro_rules! impl_from_primitive_ty {
    ($($ty:ident),*) => {
        $(
        impl FromPrimitive<$ty> for $ty {
            #[inline]
            fn from(i: $ty) -> Option<$ty> {
                Some(i)
            }
        }
        )*
    }
}
impl_from_primitive_ty!(u8, u16);

macro_rules! impl_from_primitive {
    ($name: ident, $($field: ident),*) => {
        impl ::protocol::util::FromPrimitive<u8> for $name {
            fn from(i: u8) -> Option<$name> {
                match i {
                    $( x if x == $name::$field as u8 => Some($name::$field), )*
                    _ => None
                }
            }
        }

        impl ::protocol::util::FromPrimitive<u16> for $name {
            fn from(i: u16) -> Option<$name> {
                match i {
                    $( x if x == $name::$field as u16 => Some($name::$field), )*
                    _ => None
                }
            }
        }
    }
}

macro_rules! read_packet_data {
    (None, $_self:expr,$read_fn:ident,$from_fn:ident,$msg:expr) => { read_packet_data!(None, $_self, $read_fn, $from_fn, $msg, 0) };
    (None, $_self:expr,$read_fn:ident,$from_fn:ident,$msg:expr, $line:expr) => ({
        let read_data = try!($_self.$read_fn());
        try!(::protocol::util::FromPrimitive::from(read_data).ok_or(TdsProtocolError::InvalidValue(format!($msg, read_data), $line)))
    });
    ($read_gen:ty,$_self:expr,$read_fn:ident,$from_fn:ident,$msg:expr) => { read_packet_data!($read_gen, $_self, $read_fn, $from_fn, $msg, 0) };
    ($read_gen:ty,$_self:expr,$read_fn:ident,$from_fn:ident,$msg:expr,$line:expr) => ({
        let read_data = try!($_self.$read_fn::<$read_gen>());
        try!(::protocol::util::FromPrimitive::from(read_data).ok_or(TdsProtocolError::InvalidValue(format!($msg, read_data), $line)))
    });
}
