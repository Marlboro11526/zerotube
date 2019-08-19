use crate::db::schema::sources;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "sources"]
pub struct Source {
    pub name: String,
}
