use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContadorDto {
    pub id: Uuid,
    pub nome: String,
    pub valor: i32,
    pub criado_em: NaiveDateTime,
    pub atualizado_em: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ContadorCreateDto {
    #[validate(length(min = 1, max = 200, message = "O nome é obrigatório e não pode ter mais de 200 caracteres"))]
    pub nome: String,
    
    #[serde(default)]
    pub valor: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ContadorUpdateDto {
    #[validate(length(max = 200, message = "O nome não pode ter mais de 200 caracteres"))]
    pub nome: Option<String>,
    
    pub valor: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ContadorIncrementarDto {
    #[serde(default = "default_incremento")]
    pub incremento: i32,
}

fn default_incremento() -> i32 {
    1
}
