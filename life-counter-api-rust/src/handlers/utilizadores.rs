use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use validator::Validate;

use crate::db::DbPool;
use crate::dto::{UtilizadorCreateDto, UtilizadorUpdateDto};
use crate::services::UtilizadorService;

/// GET /api/utilizadores - Lista todos os utilizadores
pub async fn get_all(pool: web::Data<DbPool>) -> impl Responder {
    match UtilizadorService::get_all(&pool).await {
        Ok(utilizadores) => HttpResponse::Ok().json(utilizadores),
        Err(e) => {
            log::error!("Erro ao obter utilizadores: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// GET /api/utilizadores/{id} - Obtém um utilizador por ID
pub async fn get_by_id(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    match UtilizadorService::get_by_id(&pool, *id).await {
        Ok(Some(utilizador)) => HttpResponse::Ok().json(utilizador),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Utilizador não encontrado"
        })),
        Err(e) => {
            log::error!("Erro ao obter utilizador {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// GET /api/utilizadores/{id}/contadores - Obtém contadores associados a um utilizador
pub async fn get_contadores(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    // First check if utilizador exists
    match UtilizadorService::get_by_id(&pool, *id).await {
        Ok(Some(_)) => {
            match UtilizadorService::get_contadores(&pool, *id).await {
                Ok(contadores) => HttpResponse::Ok().json(contadores),
                Err(e) => {
                    log::error!("Erro ao obter contadores do utilizador {}: {}", id, e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "message": "Erro interno do servidor"
                    }))
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Utilizador não encontrado"
        })),
        Err(e) => {
            log::error!("Erro ao obter utilizador {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// POST /api/utilizadores - Cria um novo utilizador
pub async fn create(
    pool: web::Data<DbPool>,
    dto: web::Json<UtilizadorCreateDto>,
) -> impl Responder {
    if let Err(e) = dto.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Dados inválidos",
            "errors": e.to_string()
        }));
    }

    match UtilizadorService::create(&pool, dto.into_inner()).await {
        Ok(utilizador) => HttpResponse::Created().json(utilizador),
        Err(e) => {
            log::error!("Erro ao criar utilizador: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// PUT /api/utilizadores/{id} - Atualiza um utilizador
pub async fn update(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
    dto: web::Json<UtilizadorUpdateDto>,
) -> impl Responder {
    if let Err(e) = dto.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Dados inválidos",
            "errors": e.to_string()
        }));
    }

    match UtilizadorService::update(&pool, *id, dto.into_inner()).await {
        Ok(Some(utilizador)) => HttpResponse::Ok().json(utilizador),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Utilizador não encontrado"
        })),
        Err(e) => {
            log::error!("Erro ao atualizar utilizador {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// DELETE /api/utilizadores/{id} - Exclui um utilizador
pub async fn delete(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    match UtilizadorService::delete(&pool, *id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Utilizador não encontrado"
        })),
        Err(e) => {
            log::error!("Erro ao excluir utilizador {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/utilizadores")
            .route("", web::get().to(get_all))
            .route("", web::post().to(create))
            .route("/{id}", web::get().to(get_by_id))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(delete))
            .route("/{id}/contadores", web::get().to(get_contadores)),
    );
}
