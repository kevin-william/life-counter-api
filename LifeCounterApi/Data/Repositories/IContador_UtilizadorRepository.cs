using LifeCounterApi.Models.Entities;

namespace LifeCounterApi.Data.Repositories;

public interface IContador_UtilizadorRepository : IRepository<Contador_Utilizador>
{
    Task<IEnumerable<Contador_Utilizador>> GetByContadorIdAsync(Guid contadorId);
    Task<IEnumerable<Contador_Utilizador>> GetByUtilizadorIdAsync(Guid utilizadorId);
}
