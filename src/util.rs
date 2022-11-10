// Rust Monero Light Wallet Server RPC Client
// Written in 2021-2022 by
//   Sebastian Kung <seb.kung@gmail.com>
//   Monero Rust Contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//

use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{self, Display};

pub trait HashType: Sized {
    fn bytes(&self) -> &[u8];
    fn from_str(v: &str) -> anyhow::Result<Self>;
}

macro_rules! hash_type_impl {
    ($name:ty) => {
        impl HashType for $name {
            fn bytes(&self) -> &[u8] {
                self.as_bytes()
            }
            fn from_str(v: &str) -> anyhow::Result<Self> {
                Ok(v.parse()?)
            }
        }
    };
}

hash_type_impl!(monero::util::address::PaymentId);
hash_type_impl!(monero::cryptonote::hash::Hash);

impl HashType for Vec<u8> {
    fn bytes(&self) -> &[u8] {
        self
    }
    fn from_str(v: &str) -> anyhow::Result<Self> {
        Ok(hex::decode(v)?)
    }
}

#[derive(Clone, Debug)]
pub struct HashString<T>(pub T);

impl<T> Display for HashString<T>
where
    T: HashType,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0.bytes()))
    }
}

impl<T> Serialize for HashString<T>
where
    T: HashType,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de, T> Deserialize<'de> for HashString<T>
where
    T: HashType,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Self(T::from_str(&s).map_err(serde::de::Error::custom)?))
    }
}
