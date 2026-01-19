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

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_contador_utilizador_create_dto_valid() {
        let dto = ContadorUtilizadorCreateDto {
            contador_id: Uuid::new_v4(),
            utilizador_id: Uuid::new_v4(),
            observacoes: Some("Teste de observação".to_string()),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_contador_utilizador_create_dto_sem_observacoes() {
        let dto = ContadorUtilizadorCreateDto {
            contador_id: Uuid::new_v4(),
            utilizador_id: Uuid::new_v4(),
            observacoes: None,
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_contador_utilizador_create_dto_observacoes_muito_longas() {
        let dto = ContadorUtilizadorCreateDto {
            contador_id: Uuid::new_v4(),
            utilizador_id: Uuid::new_v4(),
            observacoes: Some("a".repeat(501)),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_contador_utilizador_update_dto_valid() {
        let dto = ContadorUtilizadorUpdateDto {
            observacoes: Some("Nova observação".to_string()),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_contador_utilizador_update_dto_observacoes_muito_longas() {
        let dto = ContadorUtilizadorUpdateDto {
            observacoes: Some("a".repeat(501)),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_contador_utilizador_update_dto_sem_observacoes() {
        let dto = ContadorUtilizadorUpdateDto {
            observacoes: None,
        };
        assert!(dto.validate().is_ok());
    }
}
