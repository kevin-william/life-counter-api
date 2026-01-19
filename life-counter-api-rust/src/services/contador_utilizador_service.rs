use diesel::prelude::*;
use uuid::Uuid;

use crate::db::schema::contador_utilizadores;
use crate::db::DbPool;
use crate::dto::{ContadorUtilizadorCreateDto, ContadorUtilizadorDto, ContadorUtilizadorUpdateDto};
use crate::models::{ContadorUtilizador, NewContadorUtilizador};

pub struct ContadorUtilizadorService;

impl ContadorUtilizadorService {
    pub async fn get_all(pool: &DbPool) -> Result<Vec<ContadorUtilizadorDto>, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let results = contador_utilizadores::table
            .select(ContadorUtilizador::as_select())
            .load(&mut conn)
            .map_err(|e| e.to_string())?;
        
        Ok(results.into_iter().map(|cu| ContadorUtilizadorDto {
            id: cu.id,
            contador_id: cu.contador_id,
            utilizador_id: cu.utilizador_id,
            observacoes: cu.observacoes,
            criado_em: cu.criado_em,
        }).collect())
    }

    pub async fn get_by_id(pool: &DbPool, id: Uuid) -> Result<Option<ContadorUtilizadorDto>, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let result = contador_utilizadores::table
            .find(id)
            .select(ContadorUtilizador::as_select())
            .first(&mut conn)
            .optional()
            .map_err(|e| e.to_string())?;
        
        Ok(result.map(|cu| ContadorUtilizadorDto {
            id: cu.id,
            contador_id: cu.contador_id,
            utilizador_id: cu.utilizador_id,
            observacoes: cu.observacoes,
            criado_em: cu.criado_em,
        }))
    }

    pub async fn create(pool: &DbPool, dto: ContadorUtilizadorCreateDto) -> Result<ContadorUtilizadorDto, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let new_cu = NewContadorUtilizador {
            contador_id: dto.contador_id,
            utilizador_id: dto.utilizador_id,
            observacoes: dto.observacoes,
        };
        
        let result = diesel::insert_into(contador_utilizadores::table)
            .values(&new_cu)
            .returning(ContadorUtilizador::as_returning())
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;
        
        Ok(ContadorUtilizadorDto {
            id: result.id,
            contador_id: result.contador_id,
            utilizador_id: result.utilizador_id,
            observacoes: result.observacoes,
            criado_em: result.criado_em,
        })
    }

    pub async fn update(pool: &DbPool, id: Uuid, dto: ContadorUtilizadorUpdateDto) -> Result<Option<ContadorUtilizadorDto>, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let result = diesel::update(contador_utilizadores::table.find(id))
            .set(dto.observacoes.map(|o| contador_utilizadores::observacoes.eq(o)))
            .returning(ContadorUtilizador::as_returning())
            .get_result(&mut conn)
            .optional()
            .map_err(|e| e.to_string())?;
        
        Ok(result.map(|cu| ContadorUtilizadorDto {
            id: cu.id,
            contador_id: cu.contador_id,
            utilizador_id: cu.utilizador_id,
            observacoes: cu.observacoes,
            criado_em: cu.criado_em,
        }))
    }

    pub async fn delete(pool: &DbPool, id: Uuid) -> Result<bool, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let deleted = diesel::delete(contador_utilizadores::table.find(id))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        
        Ok(deleted > 0)
    }
}
