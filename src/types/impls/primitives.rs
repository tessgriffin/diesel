use persistable::AsBindParam;
use std::error::Error;
use std::io::Write;
use super::option::UnexpectedNullError;
use types::{NativeSqlType, FromSql, ToSql, IsNull};
use {Queriable, types};

macro_rules! primitive_impls {
    ($($Source:ident -> $Target:ty),+,) => {
        $(
            impl NativeSqlType for types::$Source {}

            impl Queriable<types::$Source> for $Target {
                type Row = Self;

                fn build(row: Self::Row) -> Self {
                    row
                }
            }

            impl AsBindParam<types::$Source> for $Target {
                fn as_bind_param_for_insert(&self, idx: &mut usize) -> String {
                    AsBindParam::<types::$Source>::as_bind_param(self, idx)
                }
            }
        )+
    }
}

primitive_impls! {
    Bool -> bool,

    SmallSerial -> i16,
    Serial -> i32,
    BigSerial -> i64,

    SmallInt -> i16,
    Integer -> i32,
    BigInt -> i64,

    Float -> f32,
    Double -> f64,

    VarChar -> String,

    Binary -> Vec<u8>,
}

impl FromSql<types::Bool> for bool {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error>> {
        let bytes = not_none!(bytes);
        Ok(bytes[0] != 0)
    }
}

impl ToSql<types::Bool> for bool {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        let write_result = if *self {
            out.write_all(&[1])
        } else {
            out.write_all(&[0])
        };
        write_result
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl FromSql<types::VarChar> for String {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error>> {
        let bytes = not_none!(bytes);
        String::from_utf8(bytes.into()).map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl ToSql<types::VarChar> for String {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        out.write_all(self.as_bytes())
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl<'a> ToSql<types::VarChar> for &'a str {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        out.write_all(self.as_bytes())
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl FromSql<types::Binary> for Vec<u8> {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self, Box<Error>> {
        Ok(not_none!(bytes).into())
    }
}

impl ToSql<types::Binary> for Vec<u8> {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        out.write_all(&self)
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error>)
    }
}

impl<'a> ToSql<types::Binary> for &'a [u8] {
    fn to_sql<W: Write>(&self, out: &mut W) -> Result<IsNull, Box<Error>> {
        out.write_all(self)
            .map(|_| IsNull::No)
            .map_err(|e| Box::new(e) as Box<Error>)
    }
}

#[test]
fn bool_to_sql() {
    let mut bytes = vec![];
    true.to_sql(&mut bytes).unwrap();
    false.to_sql(&mut bytes).unwrap();
    assert_eq!(bytes, vec![1u8, 0u8]);
}