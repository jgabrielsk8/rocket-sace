use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};

use crate::schema::majad_clase;

#[derive(Queryable, Serialize)]
pub struct Clase {
    id: i32,
    codigo: String,
    nombre: String,
    description: Option<String>
}

#[derive(Insertable, Deserialize)]
#[table_name = "majad_clase"]
pub struct NewClase {
    codigo: String,
    nombre: String,
    description: Option<String>
}