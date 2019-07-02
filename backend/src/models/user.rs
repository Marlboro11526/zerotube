use crate::db::entities::user::User as DbUser;
use crate::messages::error::ErrorResponse;
use argon2rs::verifier::Encoded;
use rand::RngCore;
use serde::de::{self, Deserialize, Deserializer, Error as DeError, MapAccess, SeqAccess, Visitor};
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone, Debug)]
pub struct User {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl User {
    fn new(email: String, username: String, raw_password: String) -> Result<Self, ErrorResponse> {
        let hashed_password = Self::hash_password(raw_password.as_str())?;

        Ok(User {
            email,
            username,
            password: hashed_password,
        })
    }

    fn hash_password(password: &str) -> Result<String, ErrorResponse> {
        let salt = format!("{:X}", rand::thread_rng().next_u64());
        let encoded = Encoded::default2i(password.as_bytes(), salt.as_bytes(), &[], &[]);

        String::from_utf8(encoded.to_u8()).map_err(|_| ErrorResponse::InternalServerError)
    }
}

impl From<DbUser> for User {
    fn from(entity: DbUser) -> Self {
        User {
            email: entity.email,
            username: entity.username,
            password: entity.password,
        }
    }
}

impl<'de> Deserialize<'de> for User {
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
                        formatter.write_str("`email`, `username` or `password`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "email" => Ok(Field::Email),
                            "username" => Ok(Field::Username),
                            "password" => Ok(Field::Password),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct UserVisitor;

        impl<'de> Visitor<'de> for UserVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("struct User")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<User, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let email = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(0, &self))?;

                let username = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(1, &self))?;

                let password = seq
                    .next_element()?
                    .ok_or_else(|| DeError::invalid_length(2, &self))?;

                User::new(email, username, password)
                    .ok()
                    .ok_or_else(|| DeError::custom("could not create user"))
            }

            fn visit_map<V>(self, mut map: V) -> Result<User, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut email = None;
                let mut username = None;
                let mut password = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Email => {
                            if email.is_some() {
                                return Err(DeError::duplicate_field("email"));
                            }

                            email = Some(map.next_value()?);
                        }
                        Field::Username => {
                            if username.is_some() {
                                return Err(DeError::duplicate_field("username"));
                            }

                            username = Some(map.next_value()?);
                        }
                        Field::Password => {
                            if password.is_some() {
                                return Err(DeError::duplicate_field("password"));
                            }

                            password = Some(map.next_value()?);
                        }
                    }
                }

                let email = email.ok_or_else(|| DeError::missing_field("email"))?;
                let username = username.ok_or_else(|| DeError::missing_field("username"))?;
                let password = password.ok_or_else(|| DeError::missing_field("password"))?;

                User::new(email, username, password)
                    .ok()
                    .ok_or_else(|| DeError::custom("could not create user"))
            }
        }

        const FIELDS: &'static [&'static str] = &["email", "username", "password"];

        deserializer.deserialize_struct("Duration", FIELDS, UserVisitor)
    }
}

