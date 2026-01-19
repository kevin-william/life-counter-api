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

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_utilizador_create_dto_valid() {
        let dto = UtilizadorCreateDto {
            nome: "João Silva".to_string(),
            email: "joao@example.com".to_string(),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_utilizador_create_dto_nome_vazio() {
        let dto = UtilizadorCreateDto {
            nome: "".to_string(),
            email: "joao@example.com".to_string(),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_utilizador_create_dto_email_invalido() {
        let dto = UtilizadorCreateDto {
            nome: "João Silva".to_string(),
            email: "email-invalido".to_string(),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_utilizador_create_dto_email_vazio() {
        let dto = UtilizadorCreateDto {
            nome: "João Silva".to_string(),
            email: "".to_string(),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_utilizador_create_dto_nome_muito_longo() {
        let dto = UtilizadorCreateDto {
            nome: "a".repeat(201),
            email: "joao@example.com".to_string(),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_utilizador_create_dto_email_muito_longo() {
        let dto = UtilizadorCreateDto {
            nome: "João Silva".to_string(),
            email: format!("{}@example.com", "a".repeat(190)),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_utilizador_update_dto_valid() {
        let dto = UtilizadorUpdateDto {
            nome: Some("Maria Santos".to_string()),
            email: Some("maria@example.com".to_string()),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_utilizador_update_dto_email_invalido() {
        let dto = UtilizadorUpdateDto {
            nome: None,
            email: Some("email-invalido".to_string()),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_utilizador_update_dto_campos_opcionais() {
        let dto = UtilizadorUpdateDto {
            nome: None,
            email: None,
        };
        assert!(dto.validate().is_ok());
    }
}
