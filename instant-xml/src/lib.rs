use std::collections::HashMap;
use std::fmt;
use std::collections::BTreeSet;

use thiserror::Error;
pub use xmlparser;

pub use macros::{FromXml, ToXml};

#[doc(hidden)]
pub mod parse;

pub trait ToXml {
    fn write_xml<W: fmt::Write>(
        &self,
        write: &mut W,
        parent_prefixes: Option<&mut BTreeSet<&str>>,
    ) -> Result<(), Error>;

    fn to_xml(&self, parent_prefixes: Option<&mut BTreeSet<&str>>) -> Result<String, Error> {
        let mut out = String::new();
        self.write_xml(&mut out, parent_prefixes)?;
        Ok(out)
    }
}

impl ToXml for bool {
    fn write_xml<W: fmt::Write>(
        &self,
        _write: &mut W,
        _parent_prefixes: Option<&mut BTreeSet<&str>>,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn to_xml(&self, parent_prefixes: Option<&mut BTreeSet<&str>>) -> Result<String, Error> {
        let mut out = self.to_string();
        self.write_xml(&mut out, parent_prefixes)?;
        Ok(out)
    }
}

pub trait FromXml<'xml>: Sized {
    fn from_xml(input: &str) -> Result<Self, Error>;
}

pub trait FromXmlOwned: for<'xml> FromXml<'xml> {}

#[allow(dead_code)]
pub struct State<'a> {
    prefix: HashMap<&'a str, &'a str>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("format: {0}")]
    Format(#[from] fmt::Error),
    #[error("parse: {0}")]
    Parse(#[from] xmlparser::Error),
    #[error("unexpected end of stream")]
    UnexpectedEndOfStream,
    #[error("unexpected value")]
    UnexpectedValue,
}
