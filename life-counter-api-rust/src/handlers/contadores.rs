use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use validator::Validate;

use crate::db::DbPool;
use crate::dto::{ContadorCreateDto, ContadorUpdateDto, ContadorIncrementarDto};
use crate::services::ContadorService;

/// GET /api/contadores - Lista todos os contadores
pub async fn get_all(pool: web::Data<DbPool>) -> impl Responder {
    match ContadorService::get_all(&pool).await {
        Ok(contadores) => HttpResponse::Ok().json(contadores),
        Err(e) => {
            log::error!("Erro ao obter contadores: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// GET /api/contadores/{id} - Obtém um contador por ID
pub async fn get_by_id(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    match ContadorService::get_by_id(&pool, *id).await {
        Ok(Some(contador)) => HttpResponse::Ok().json(contador),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Contador não encontrado"
        })),
        Err(e) => {
            log::error!("Erro ao obter contador {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// GET /api/contadores/{id}/utilizadores - Obtém utilizadores associados a um contador
pub async fn get_utilizadores(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    // First check if contador exists
    match ContadorService::get_by_id(&pool, *id).await {
        Ok(Some(_)) => {
            match ContadorService::get_utilizadores(&pool, *id).await {
                Ok(utilizadores) => HttpResponse::Ok().json(utilizadores),
                Err(e) => {
                    log::error!("Erro ao obter utilizadores do contador {}: {}", id, e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "message": "Erro interno do servidor"
                    }))
                }
            }
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Contador não encontrado"
        })),
        Err(e) => {
            log::error!("Erro ao obter contador {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// POST /api/contadores - Cria um novo contador
pub async fn create(
    pool: web::Data<DbPool>,
    dto: web::Json<ContadorCreateDto>,
) -> impl Responder {
    if let Err(e) = dto.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Dados inválidos",
            "errors": e.to_string()
        }));
    }

    match ContadorService::create(&pool, dto.into_inner()).await {
        Ok(contador) => HttpResponse::Created().json(contador),
        Err(e) => {
            log::error!("Erro ao criar contador: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// PUT /api/contadores/{id} - Atualiza um contador
pub async fn update(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
    dto: web::Json<ContadorUpdateDto>,
) -> impl Responder {
    if let Err(e) = dto.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Dados inválidos",
            "errors": e.to_string()
        }));
    }

    match ContadorService::update(&pool, *id, dto.into_inner()).await {
        Ok(Some(contador)) => HttpResponse::Ok().json(contador),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Contador não encontrado"
        })),
        Err(e) => {
            log::error!("Erro ao atualizar contador {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// PATCH /api/contadores/{id}/incrementar - Incrementa o valor de um contador
pub async fn incrementar(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
    dto: web::Json<ContadorIncrementarDto>,
) -> impl Responder {
    if let Err(e) = dto.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Dados inválidos",
            "errors": e.to_string()
        }));
    }

    match ContadorService::incrementar(&pool, *id, dto.incremento).await {
        Ok(Some(contador)) => HttpResponse::Ok().json(contador),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Contador não encontrado"
        })),
        Err(e) => {
            log::error!("Erro ao incrementar contador {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

/// DELETE /api/contadores/{id} - Exclui um contador
pub async fn delete(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    match ContadorService::delete(&pool, *id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Contador não encontrado"
        })),
        Err(e) => {
            log::error!("Erro ao excluir contador {}: {}", id, e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Erro interno do servidor"
            }))
        }
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/contadores")
            .route("", web::get().to(get_all))
            .route("", web::post().to(create))
            .route("/{id}", web::get().to(get_by_id))
            .route("/{id}", web::put().to(update))
            .route("/{id}", web::delete().to(delete))
            .route("/{id}/utilizadores", web::get().to(get_utilizadores))
            .route("/{id}/incrementar", web::patch().to(incrementar)),
    );
}
