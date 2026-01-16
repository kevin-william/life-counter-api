using LifeCounterApi.Models.Entities;
using Microsoft.EntityFrameworkCore;

namespace LifeCounterApi.Data.Repositories;

public class UtilizadorRepository : Repository<Utilizador>, IUtilizadorRepository
{
    public UtilizadorRepository(ApplicationDbContext context, ILogger<Repository<Utilizador>> logger) 
        : base(context, logger)
    {
    }

    public async Task<IEnumerable<Contador>> GetContadoresByUtilizadorIdAsync(Guid utilizadorId)
    {
        try
        {
            return await _context.Contador_Utilizadores
                .Where(cu => cu.UtilizadorId == utilizadorId)
                .Include(cu => cu.Contador)
                .Select(cu => cu.Contador)
                .ToListAsync();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao buscar contadores do utilizador: {UtilizadorId}", utilizadorId);
            throw;
        }
    }

    public async Task<Utilizador?> GetByEmailAsync(string email)
    {
        try
        {
            return await _dbSet.FirstOrDefaultAsync(u => u.Email == email);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao buscar utilizador por email: {Email}", email);
            throw;
        }
    }
}
