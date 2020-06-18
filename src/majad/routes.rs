use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use rocket_contrib::json::Json;

use crate::DbConn;
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




#[post("/clases", data = "<add_clase>")]
pub fn create_clase(conn: DbConn, add_clase: Json<NewClase>) -> Json<Clase> {
    let result = diesel::insert_into(majad_clase::table)
        .values(&add_clase.0)
        .get_result(&*conn)
        .unwrap();

    Json(result)
}

#[get("/clases")]
pub fn get_clases(conn: DbConn) -> Json<Vec<Clase>> {
    let clases = majad_clase::table
        .order(majad_clase::columns::id.desc())
        .load::<Clase>(&*conn)
        .unwrap();

    Json(clases)
}