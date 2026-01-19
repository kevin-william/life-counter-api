use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct UtilizadorDto {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub criado_em: NaiveDateTime,
    pub atualizado_em: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UtilizadorCreateDto {
    #[validate(length(min = 1, max = 200, message = "O nome é obrigatório e não pode ter mais de 200 caracteres"))]
    pub nome: String,
    
    #[validate(email(message = "Email inválido"))]
    #[validate(length(min = 1, max = 200, message = "O email é obrigatório e não pode ter mais de 200 caracteres"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UtilizadorUpdateDto {
    #[validate(length(max = 200, message = "O nome não pode ter mais de 200 caracteres"))]
    pub nome: Option<String>,
    
    #[validate(email(message = "Email inválido"))]
    #[validate(length(max = 200, message = "O email não pode ter mais de 200 caracteres"))]
    pub email: Option<String>,
}
