using LifeCounterApi.Models.Entities;

namespace LifeCounterApi.Data.Repositories;

public interface IContadorRepository : IRepository<Contador>
{
    Task<IEnumerable<Utilizador>> GetUtilizadoresByContadorIdAsync(Guid contadorId);
    Task<Contador?> IncrementarAsync(Guid id, int incremento);
}
