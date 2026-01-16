using LifeCounterApi.Models.Entities;

namespace LifeCounterApi.Data.Repositories;

public interface IUtilizadorRepository : IRepository<Utilizador>
{
    Task<IEnumerable<Contador>> GetContadoresByUtilizadorIdAsync(Guid utilizadorId);
    Task<Utilizador?> GetByEmailAsync(string email);
}
