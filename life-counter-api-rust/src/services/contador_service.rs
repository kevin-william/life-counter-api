use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::schema::{contadores, contador_utilizadores, utilizadores};
use crate::db::DbPool;
use crate::dto::{ContadorCreateDto, ContadorDto, ContadorUpdateDto, UtilizadorDto};
use crate::models::{Contador, NewContador, Utilizador};

pub struct ContadorService;

impl ContadorService {
    pub async fn get_all(pool: &DbPool) -> Result<Vec<ContadorDto>, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let results = contadores::table
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

    pub async fn get_by_id(pool: &DbPool, id: Uuid) -> Result<Option<ContadorDto>, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let result = contadores::table
            .find(id)
            .select(Contador::as_select())
            .first(&mut conn)
            .optional()
            .map_err(|e| e.to_string())?;
        
        Ok(result.map(|c| ContadorDto {
            id: c.id,
            nome: c.nome,
            valor: c.valor,
            criado_em: c.criado_em,
            atualizado_em: c.atualizado_em,
        }))
    }

    pub async fn get_utilizadores(pool: &DbPool, contador_id: Uuid) -> Result<Vec<UtilizadorDto>, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let results = contador_utilizadores::table
            .inner_join(utilizadores::table)
            .filter(contador_utilizadores::contador_id.eq(contador_id))
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

    pub async fn create(pool: &DbPool, dto: ContadorCreateDto) -> Result<ContadorDto, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let new_contador = NewContador {
            nome: dto.nome,
            valor: dto.valor,
        };
        
        let result = diesel::insert_into(contadores::table)
            .values(&new_contador)
            .returning(Contador::as_returning())
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;
        
        Ok(ContadorDto {
            id: result.id,
            nome: result.nome,
            valor: result.valor,
            criado_em: result.criado_em,
            atualizado_em: result.atualizado_em,
        })
    }

    pub async fn update(pool: &DbPool, id: Uuid, dto: ContadorUpdateDto) -> Result<Option<ContadorDto>, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let now = Utc::now().naive_utc();
        
        let result = diesel::update(contadores::table.find(id))
            .set((
                dto.nome.map(|n| contadores::nome.eq(n)),
                dto.valor.map(|v| contadores::valor.eq(v)),
                Some(contadores::atualizado_em.eq(now)),
            ))
            .returning(Contador::as_returning())
            .get_result(&mut conn)
            .optional()
            .map_err(|e| e.to_string())?;
        
        Ok(result.map(|c| ContadorDto {
            id: c.id,
            nome: c.nome,
            valor: c.valor,
            criado_em: c.criado_em,
            atualizado_em: c.atualizado_em,
        }))
    }

    pub async fn incrementar(pool: &DbPool, id: Uuid, incremento: i32) -> Result<Option<ContadorDto>, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let now = Utc::now().naive_utc();
        
        let result = diesel::update(contadores::table.find(id))
            .set((
                contadores::valor.eq(contadores::valor + incremento),
                contadores::atualizado_em.eq(now),
            ))
            .returning(Contador::as_returning())
            .get_result(&mut conn)
            .optional()
            .map_err(|e| e.to_string())?;
        
        Ok(result.map(|c| ContadorDto {
            id: c.id,
            nome: c.nome,
            valor: c.valor,
            criado_em: c.criado_em,
            atualizado_em: c.atualizado_em,
        }))
    }

    pub async fn delete(pool: &DbPool, id: Uuid) -> Result<bool, String> {
        let mut conn = pool.get().map_err(|e| e.to_string())?;
        
        let deleted = diesel::delete(contadores::table.find(id))
            .execute(&mut conn)
            .map_err(|e| e.to_string())?;
        
        Ok(deleted > 0)
    }
}
