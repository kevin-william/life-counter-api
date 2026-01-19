use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::schema::{utilizadores, contador_utilizadores, contadores};
use crate::db::DbPool;
use crate::dto::{UtilizadorCreateDto, UtilizadorDto, UtilizadorUpdateDto, ContadorDto};
use crate::models::{Utilizador, NewUtilizador, Contador};

pub struct UtilizadorService;

impl UtilizadorService {
    pub async fn get_all(pool: &DbPool) -> Result<Vec<UtilizadorDto>, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let results = utilizadores::table
            .select(Utilizador::as_select())
            .load(&mut conn)
            .map_err(|e| e.to_string())?;
        
        Ok(results.into_iter().map(|u| UtilizadorDto {
            id: u.id,
            nome: u.nome,
            email: u.email,
            criado_em: u.criado_em,
            atualizado_em: u.atualizado_em,
        }).collect())
    }

    pub async fn get_by_id(pool: &DbPool, id: Uuid) -> Result<Option<UtilizadorDto>, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let result = utilizadores::table
            .find(id)
            .select(Utilizador::as_select())
            .first(&mut conn)
            .optional()
            .map_err(|e| e.to_string())?;
        
        Ok(result.map(|u| UtilizadorDto {
            id: u.id,
            nome: u.nome,
            email: u.email,
            criado_em: u.criado_em,
            atualizado_em: u.atualizado_em,
        }))
    }

    pub async fn get_contadores(pool: &DbPool, utilizador_id: Uuid) -> Result<Vec<ContadorDto>, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let results = contador_utilizadores::table
            .inner_join(contadores::table)
            .filter(contador_utilizadores::utilizador_id.eq(utilizador_id))
            .select(Contador::as_select())
            .load(&mut conn)
            .map_err(|e| e.to_string())?;
        
        Ok(results.into_iter().map(|c| ContadorDto {
            id: c.id,
            nome: c.nome,
            valor: c.valor,
            criado_em: c.criado_em,
            atualizado_em: c.atualizado_em,
        }).collect())
    }

    pub async fn create(pool: &DbPool, dto: UtilizadorCreateDto) -> Result<UtilizadorDto, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let new_utilizador = NewUtilizador {
            nome: dto.nome,
            email: dto.email,
        };
        
        let result = diesel::insert_into(utilizadores::table)
            .values(&new_utilizador)
            .returning(Utilizador::as_returning())
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;
        
        Ok(UtilizadorDto {
            id: result.id,
            nome: result.nome,
            email: result.email,
            criado_em: result.criado_em,
            atualizado_em: result.atualizado_em,
        })
    }

    pub async fn update(pool: &DbPool, id: Uuid, dto: UtilizadorUpdateDto) -> Result<Option<UtilizadorDto>, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let now = Utc::now().naive_utc();
        
        let result = diesel::update(utilizadores::table.find(id))
            .set((
                dto.nome.map(|n| utilizadores::nome.eq(n)),
                dto.email.map(|e| utilizadores::email.eq(e)),
                Some(utilizadores::atualizado_em.eq(now)),
            ))
            .returning(Utilizador::as_returning())
            .get_result(&mut conn)
            .optional()
            .map_err(|e| e.to_string())?;
        
        Ok(result.map(|u| UtilizadorDto {
            id: u.id,
            nome: u.nome,
            email: u.email,
            criado_em: u.criado_em,
            atualizado_em: u.atualizado_em,
        }))
    }

    pub async fn delete(pool: &DbPool, id: Uuid) -> Result<bool, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let deleted = diesel::delete(utilizadores::table.find(id))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        
        Ok(deleted > 0)
    }
}
