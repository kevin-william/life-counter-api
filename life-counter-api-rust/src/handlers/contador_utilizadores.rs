use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use validator::Validate;

use crate::db::DbPool;
use crate::dto::{ContadorUtilizadorCreateDto, ContadorUtilizadorUpdateDto};
use crate::services::ContadorUtilizadorService;

/// GET /api/contador_utilizadores - Lista todas as associações
pub async fn get_all(pool: web::Data<DbPool>) -> impl Responder {
    match ContadorUtilizadorService::get_all(&pool).await {
        Ok(associacoes) => HttpResponse::Ok().json(associacoes),
        Err(e) => {
            log::error!("Erro ao obter associações: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// GET /api/contador_utilizadores/{id} - Obtém uma associação por ID
pub async fn get_by_id(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    match ContadorUtilizadorService::get_by_id(&pool, *id).await {
        Ok(Some(associacao)) => HttpResponse::Ok().json(associacao),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Associação não encontrada"
        })),
        Err(e) => {
            log::error!("Erro ao obter associação {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// POST /api/contador_utilizadores - Cria uma nova associação
pub async fn create(
    pool: web::Data<DbPool>,
    dto: web::Json<ContadorUtilizadorCreateDto>,
) -> impl Responder {
    if let Err(e) = dto.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Dados inválidos",
            "errors": e.to_string()
        }));
    }

    match ContadorUtilizadorService::create(&pool, dto.into_inner()).await {
        Ok(associacao) => HttpResponse::Created().json(associacao),
        Err(e) => {
            log::error!("Erro ao criar associação: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// PUT /api/contador_utilizadores/{id} - Atualiza uma associação
pub async fn update(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
    dto: web::Json<ContadorUtilizadorUpdateDto>,
) -> impl Responder {
    if let Err(e) = dto.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Dados inválidos",
            "errors": e.to_string()
        }));
    }

    match ContadorUtilizadorService::update(&pool, *id, dto.into_inner()).await {
        Ok(Some(associacao)) => HttpResponse::Ok().json(associacao),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Associação não encontrada"
        })),
        Err(e) => {
            log::error!("Erro ao atualizar associação {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// DELETE /api/contador_utilizadores/{id} - Exclui uma associação
pub async fn delete(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    match ContadorUtilizadorService::delete(&pool, *id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Associação não encontrada"
        })),
        Err(e) => {
            log::error!("Erro ao excluir associação {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/contador_utilizadores")
            .route("", web::get().to(get_all))
            .route("", web::post().to(create))
            .route("/{id}", web::get().to(get_by_id))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(delete)),
    );
}
