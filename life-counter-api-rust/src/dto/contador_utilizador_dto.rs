use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContadorUtilizadorDto {
    pub id: Uuid,
    pub contador_id: Uuid,
    pub utilizador_id: Uuid,
    pub observacoes: Option<String>,
    pub criado_em: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ContadorUtilizadorCreateDto {
    pub contador_id: Uuid,
    pub utilizador_id: Uuid,
    
    #[validate(length(max = 500, message = "As observações não podem ter mais de 500 caracteres"))]
    pub observacoes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ContadorUtilizadorUpdateDto {
    #[validate(length(max = 500, message = "As observações não podem ter mais de 500 caracteres"))]
    pub observacoes: Option<String>,
}
