use axum::Json;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::Deserialize;
use sqlx::types::JsonValue;
use std::convert::TryFrom;

#[derive(Debug, IntoPrimitive, TryFromPrimitive, Clone, Deserialize)]
#[repr(u8)]
pub enum Permission {
    All = 1,
    AdminUser = 2,
    EditRole = 3,
    Unknown,
}

impl From<&String> for Permission {
    fn from(s: &String) -> Permission {
        match s.as_str() {
            "All" => Permission::All,
            "AdminUser" => Permission::AdminUser,
            "EditRole" => Permission::EditRole,
            _ => Permission::Unknown,
        }
    }
}

impl Into<String> for Permission {
    fn into(self) -> String {
        let x = match self {
            Permission::All => "All",
            Permission::AdminUser => "AdminUser",
            Permission::EditRole => "EditRole",
            _ => "Unknown",
        };
        x.into()
    }
}

impl Permission {
    pub fn get_range() -> (u8, u8) {
        (Permission::All.into(), Permission::Unknown.into())
    }

    pub fn all_labels() -> Vec<String> {
        let (start, end) = Permission::get_range();
        let labels = (start..end)
            .map(|x| {
                let p = Permission::try_from(x).unwrap();
                p.into()
            })
            .collect();
        labels
    }

    pub fn from_json_value(x: &Option<JsonValue>) -> Option<Vec<Permission>> {
        x.as_ref()?
            .as_array()?
            .iter()
            .map(|elm| {
                let p = elm.as_i64().unwrap() as u8;
                let p = Permission::try_from(p).ok()?;
                Some(p)
            })
            .collect()
    }

    pub fn to_json_value(names: &[String]) -> JsonValue {
        let p = names
            .iter()
            .map(|p| {
                let p: Permission = p.into();
                p.into()
            })
            .collect::<Vec<u8>>();
        serde_json::value::to_value(p).unwrap()
    }

    pub fn json_value_to_names(x: &Option<JsonValue>) -> Option<Vec<String>> {
        x.as_ref()?
            .as_array()?
            .iter()
            .map(|elm| {
                let p = elm.as_i64().unwrap() as u8;
                let p = Permission::try_from(p).ok()?;
                Some(p.into())
            })
            .collect()
    }
}
