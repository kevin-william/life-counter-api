use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::contador_utilizadores;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = contador_utilizadores)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ContadorUtilizador {
    pub id: Uuid,
    pub contador_id: Uuid,
    pub utilizador_id: Uuid,
    pub observacoes: Option<String>,
    pub criado_em: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = contador_utilizadores)]
pub struct NewContadorUtilizador {
    pub contador_id: Uuid,
    pub utilizador_id: Uuid,
    pub observacoes: Option<String>,
}
