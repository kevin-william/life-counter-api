using LifeCounterApi.Models.Entities;
using Microsoft.EntityFrameworkCore;

namespace LifeCounterApi.Data.Repositories;

public class Contador_UtilizadorRepository : Repository<Contador_Utilizador>, IContador_UtilizadorRepository
{
    public Contador_UtilizadorRepository(ApplicationDbContext context, ILogger<Repository<Contador_Utilizador>> logger) 
        : base(context, logger)
    {
    }

    public override async Task<IEnumerable<Contador_Utilizador>> GetAllAsync()
    {
        try
        {
            return await _dbSet
                .Include(cu => cu.Contador)
                .Include(cu => cu.Utilizador)
                .ToListAsync();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao buscar todas as associações");
            throw;
        }
    }

    public override async Task<Contador_Utilizador?> GetByIdAsync(Guid id)
    {
        try
        {
            return await _dbSet
                .Include(cu => cu.Contador)
                .Include(cu => cu.Utilizador)
                .FirstOrDefaultAsync(cu => cu.Id == id);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao buscar associação por ID: {Id}", id);
            throw;
        }
    }

    public async Task<IEnumerable<Contador_Utilizador>> GetByContadorIdAsync(Guid contadorId)
    {
        try
        {
            return await _dbSet
                .Where(cu => cu.ContadorId == contadorId)
                .Include(cu => cu.Contador)
                .Include(cu => cu.Utilizador)
                .ToListAsync();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao buscar associações por contador: {ContadorId}", contadorId);
            throw;
        }
    }

    public async Task<IEnumerable<Contador_Utilizador>> GetByUtilizadorIdAsync(Guid utilizadorId)
    {
        try
        {
            return await _dbSet
                .Where(cu => cu.UtilizadorId == utilizadorId)
                .Include(cu => cu.Contador)
                .Include(cu => cu.Utilizador)
                .ToListAsync();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao buscar associações por utilizador: {UtilizadorId}", utilizadorId);
            throw;
        }
    }
}
