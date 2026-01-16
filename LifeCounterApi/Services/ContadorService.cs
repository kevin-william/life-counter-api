using AutoMapper;
using LifeCounterApi.Data.Repositories;
using LifeCounterApi.Models.DTOs;
using LifeCounterApi.Models.Entities;

namespace LifeCounterApi.Services;

public interface IContadorService
{
    Task<IEnumerable<ContadorDto>> GetAllAsync();
    Task<ContadorDto?> GetByIdAsync(Guid id);
    Task<ContadorDto> CreateAsync(ContadorCreateDto createDto);
    Task<ContadorDto?> UpdateAsync(Guid id, ContadorUpdateDto updateDto);
    Task<bool> DeleteAsync(Guid id);
    Task<IEnumerable<UtilizadorDto>> GetUtilizadoresAsync(Guid contadorId);
    Task<ContadorDto?> IncrementarAsync(Guid id, int incremento);
}

public class ContadorService : IContadorService
{
    private readonly IContadorRepository _repository;
    private readonly IMapper _mapper;
    private readonly ILogger<ContadorService> _logger;

    public ContadorService(IContadorRepository repository, IMapper mapper, ILogger<ContadorService> logger)
    {
        _repository = repository;
        _mapper = mapper;
        _logger = logger;
    }

    public async Task<IEnumerable<ContadorDto>> GetAllAsync()
    {
        try
        {
            var contadores = await _repository.GetAllAsync();
            return _mapper.Map<IEnumerable<ContadorDto>>(contadores);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter todos os contadores");
            throw;
        }
    }

    public async Task<ContadorDto?> GetByIdAsync(Guid id)
    {
        try
        {
            var contador = await _repository.GetByIdAsync(id);
            return contador == null ? null : _mapper.Map<ContadorDto>(contador);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter contador por ID: {Id}", id);
            throw;
        }
    }

    public async Task<ContadorDto> CreateAsync(ContadorCreateDto createDto)
    {
        try
        {
            var contador = _mapper.Map<Contador>(createDto);
            contador.Id = Guid.NewGuid();
            contador.CriadoEm = DateTime.UtcNow;
            contador.AtualizadoEm = DateTime.UtcNow;
            
            var created = await _repository.CreateAsync(contador);
            return _mapper.Map<ContadorDto>(created);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao criar contador");
            throw;
        }
    }

    public async Task<ContadorDto?> UpdateAsync(Guid id, ContadorUpdateDto updateDto)
    {
        try
        {
            var contador = await _repository.GetByIdAsync(id);
            if (contador == null)
            {
                return null;
            }

            _mapper.Map(updateDto, contador);
            contador.AtualizadoEm = DateTime.UtcNow;
            
            var updated = await _repository.UpdateAsync(contador);
            return _mapper.Map<ContadorDto>(updated);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao atualizar contador: {Id}", id);
            throw;
        }
    }

    public async Task<bool> DeleteAsync(Guid id)
    {
        try
        {
            return await _repository.DeleteAsync(id);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao excluir contador: {Id}", id);
            throw;
        }
    }

    public async Task<IEnumerable<UtilizadorDto>> GetUtilizadoresAsync(Guid contadorId)
    {
        try
        {
            var utilizadores = await _repository.GetUtilizadoresByContadorIdAsync(contadorId);
            return _mapper.Map<IEnumerable<UtilizadorDto>>(utilizadores);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter utilizadores do contador: {ContadorId}", contadorId);
            throw;
        }
    }

    public async Task<ContadorDto?> IncrementarAsync(Guid id, int incremento)
    {
        try
        {
            var contador = await _repository.IncrementarAsync(id, incremento);
            return contador == null ? null : _mapper.Map<ContadorDto>(contador);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao incrementar contador: {Id}", id);
            throw;
        }
    }
}
