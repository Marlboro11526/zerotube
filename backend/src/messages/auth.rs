use crate::messages::error::ErrorResponse;
use crate::util;
use serde::de::{self, Deserialize, Deserializer, Error as DeError, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize as DeserializeDerive, Serialize};
use std::fmt::{Formatter, Result as FmtResult};

#[derive(DeserializeDerive)]
pub struct LoginRequest {
    pub password: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub username: Option<String>,
}

pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: String,
}

impl RegisterRequest {
    fn new(email: String, raw_password: String, username: String) -> Result<Self, ErrorResponse> {
        let hashed_password = util::hash_password(raw_password.as_str())?;

        Ok(RegisterRequest {
            email,
            password: hashed_password,
            username,
        })
    }
}

impl<'de> Deserialize<'de> for RegisterRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Email,
            Password,
            Username,
        };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                        formatter.write_str("`email`, `password` or `username`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "email" => Ok(Field::Email),
                            "password" => Ok(Field::Password),
                            "username" => Ok(Field::Username),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct RegisterRequestVisitor;

        impl<'de> Visitor<'de> for RegisterRequestVisitor {
            type Value = RegisterRequest;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("struct RegisterRequest")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<RegisterRequest, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let email = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(0, &self))?;

                let password = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(1, &self))?;

                let username = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(2, &self))?;

                RegisterRequest::new(email, password, username)
                    .ok()
                    .ok_or_else(|| DeError::custom("could not create user"))
            }

            fn visit_map<V>(self, mut map: V) -> Result<RegisterRequest, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut email = None;
                let mut password = None;
                let mut username = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Email => {
                            if email.is_some() {
                                return Err(DeError::duplicate_field("email"));
                            }

                            email = Some(map.next_value()?);
                        }
                        Field::Password => {
                            if password.is_some() {
                                return Err(DeError::duplicate_field("password"));
                            }

                            password = Some(map.next_value()?);
                        }
                        Field::Username => {
                            if username.is_some() {
                                return Err(DeError::duplicate_field("username"));
                            }

                            username = Some(map.next_value()?);
                        }
                    }
                }

                let email = email.ok_or_else(|| DeError::missing_field("email"))?;
                let password = password.ok_or_else(|| DeError::missing_field("password"))?;
                let username = username.ok_or_else(|| DeError::missing_field("username"))?;

                RegisterRequest::new(email, password, username)
                    .ok()
                    .ok_or_else(|| DeError::custom("could not create user"))
            }
        }

        const FIELDS: &[&str; 3] = &["email", "password", "username"];

        deserializer.deserialize_struct("Duration", FIELDS, RegisterRequestVisitor)
    }
}