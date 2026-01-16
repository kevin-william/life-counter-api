using AutoMapper;
using LifeCounterApi.Data.Repositories;
using LifeCounterApi.Models.DTOs;
using LifeCounterApi.Models.Entities;

namespace LifeCounterApi.Services;

public interface IUtilizadorService
{
    Task<IEnumerable<UtilizadorDto>> GetAllAsync();
    Task<UtilizadorDto?> GetByIdAsync(Guid id);
    Task<UtilizadorDto> CreateAsync(UtilizadorCreateDto createDto);
    Task<UtilizadorDto?> UpdateAsync(Guid id, UtilizadorUpdateDto updateDto);
    Task<bool> DeleteAsync(Guid id);
    Task<IEnumerable<ContadorDto>> GetContadoresAsync(Guid utilizadorId);
}

public class UtilizadorService : IUtilizadorService
{
    private readonly IUtilizadorRepository _repository;
    private readonly IMapper _mapper;
    private readonly ILogger<UtilizadorService> _logger;

    public UtilizadorService(IUtilizadorRepository repository, IMapper mapper, ILogger<UtilizadorService> logger)
    {
        _repository = repository;
        _mapper = mapper;
        _logger = logger;
    }

    public async Task<IEnumerable<UtilizadorDto>> GetAllAsync()
    {
        try
        {
            var utilizadores = await _repository.GetAllAsync();
            return _mapper.Map<IEnumerable<UtilizadorDto>>(utilizadores);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter todos os utilizadores");
            throw;
        }
    }

    public async Task<UtilizadorDto?> GetByIdAsync(Guid id)
    {
        try
        {
            var utilizador = await _repository.GetByIdAsync(id);
            return utilizador == null ? null : _mapper.Map<UtilizadorDto>(utilizador);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter utilizador por ID: {Id}", id);
            throw;
        }
    }

    public async Task<UtilizadorDto> CreateAsync(UtilizadorCreateDto createDto)
    {
        try
        {
            // Verificar se email já existe
            var existingUser = await _repository.GetByEmailAsync(createDto.Email);
            if (existingUser != null)
            {
                throw new InvalidOperationException("Já existe um utilizador com este email");
            }

            var utilizador = _mapper.Map<Utilizador>(createDto);
            utilizador.Id = Guid.NewGuid();
            utilizador.CriadoEm = DateTime.UtcNow;
            utilizador.AtualizadoEm = DateTime.UtcNow;
            
            var created = await _repository.CreateAsync(utilizador);
            return _mapper.Map<UtilizadorDto>(created);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao criar utilizador");
            throw;
        }
    }

    public async Task<UtilizadorDto?> UpdateAsync(Guid id, UtilizadorUpdateDto updateDto)
    {
        try
        {
            var utilizador = await _repository.GetByIdAsync(id);
            if (utilizador == null)
            {
                return null;
            }

            // Verificar se novo email já existe
            if (!string.IsNullOrEmpty(updateDto.Email) && updateDto.Email != utilizador.Email)
            {
                var existingUser = await _repository.GetByEmailAsync(updateDto.Email);
                if (existingUser != null)
                {
                    throw new InvalidOperationException("Já existe um utilizador com este email");
                }
            }

            _mapper.Map(updateDto, utilizador);
            utilizador.AtualizadoEm = DateTime.UtcNow;
            
            var updated = await _repository.UpdateAsync(utilizador);
            return _mapper.Map<UtilizadorDto>(updated);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao atualizar utilizador: {Id}", id);
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
            _logger.LogError(ex, "Erro ao excluir utilizador: {Id}", id);
            throw;
        }
    }

    public async Task<IEnumerable<ContadorDto>> GetContadoresAsync(Guid utilizadorId)
    {
        try
        {
            var contadores = await _repository.GetContadoresByUtilizadorIdAsync(utilizadorId);
            return _mapper.Map<IEnumerable<ContadorDto>>(contadores);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter contadores do utilizador: {UtilizadorId}", utilizadorId);
            throw;
        }
    }
}
