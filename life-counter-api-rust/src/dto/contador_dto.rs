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

#[derive(Debug, Deserialize, Validate)]
pub struct ContadorIncrementarDto {
    #[serde(default = "default_incremento")]
    pub incremento: i32,
}

fn default_incremento() -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_contador_create_dto_valid() {
        let dto = ContadorCreateDto {
            nome: "Contador de Teste".to_string(),
            valor: 10,
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_contador_create_dto_nome_vazio() {
        let dto = ContadorCreateDto {
            nome: "".to_string(),
            valor: 0,
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_contador_create_dto_nome_muito_longo() {
        let dto = ContadorCreateDto {
            nome: "a".repeat(201),
            valor: 0,
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_contador_create_dto_valor_default() {
        let json = r#"{"nome":"Teste"}"#;
        let dto: ContadorCreateDto = serde_json::from_str(json).unwrap();
        assert_eq!(dto.valor, 0);
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_contador_update_dto_valid() {
        let dto = ContadorUpdateDto {
            nome: Some("Novo Nome".to_string()),
            valor: Some(20),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_contador_update_dto_nome_muito_longo() {
        let dto = ContadorUpdateDto {
            nome: Some("a".repeat(201)),
            valor: None,
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_contador_update_dto_campos_opcionais() {
        let dto = ContadorUpdateDto {
            nome: None,
            valor: None,
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_contador_incrementar_dto_default() {
        let json = r#"{}"#;
        let dto: ContadorIncrementarDto = serde_json::from_str(json).unwrap();
        assert_eq!(dto.incremento, 1);
    }

    #[test]
    fn test_contador_incrementar_dto_custom_value() {
        let json = r#"{"incremento":5}"#;
        let dto: ContadorIncrementarDto = serde_json::from_str(json).unwrap();
        assert_eq!(dto.incremento, 5);
    }
}
