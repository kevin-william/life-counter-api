use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::utilizadores;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = utilizadores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Utilizador {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub criado_em: NaiveDateTime,
    pub atualizado_em: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = utilizadores)]
pub struct NewUtilizador {
    pub nome: String,
    pub email: String,
}
