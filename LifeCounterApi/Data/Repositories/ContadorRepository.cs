using LifeCounterApi.Models.Entities;
using Microsoft.EntityFrameworkCore;

namespace LifeCounterApi.Data.Repositories;

public class ContadorRepository : Repository<Contador>, IContadorRepository
{
    public ContadorRepository(ApplicationDbContext context, ILogger<Repository<Contador>> logger) 
        : base(context, logger)
    {
    }

    public async Task<IEnumerable<Utilizador>> GetUtilizadoresByContadorIdAsync(Guid contadorId)
    {
        try
        {
            return await _context.Contador_Utilizadores
                .Where(cu => cu.ContadorId == contadorId)
                .Include(cu => cu.Utilizador)
                .Select(cu => cu.Utilizador)
                .ToListAsync();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao buscar utilizadores do contador: {ContadorId}", contadorId);
            throw;
        }
    }

    public async Task<Contador?> IncrementarAsync(Guid id, int incremento)
    {
        try
        {
            var contador = await GetByIdAsync(id);
            if (contador == null)
            {
                return null;
            }

            contador.Valor += incremento;
            contador.AtualizadoEm = DateTime.UtcNow;
            
            await _context.SaveChangesAsync();
            return contador;
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao incrementar contador: {Id}", id);
            throw;
        }
    }
}
