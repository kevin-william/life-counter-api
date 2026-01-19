use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::contadores;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = contadores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Contador {
    pub id: Uuid,
    pub nome: String,
    pub valor: i32,
    pub criado_em: NaiveDateTime,
    pub atualizado_em: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = contadores)]
pub struct NewContador {
    pub nome: String,
    pub valor: i32,
}
