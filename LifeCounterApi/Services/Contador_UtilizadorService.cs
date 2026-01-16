using AutoMapper;
using LifeCounterApi.Data.Repositories;
using LifeCounterApi.Models.DTOs;
using LifeCounterApi.Models.Entities;

namespace LifeCounterApi.Services;

public interface IContador_UtilizadorService
{
    Task<IEnumerable<Contador_UtilizadorDto>> GetAllAsync();
    Task<Contador_UtilizadorDto?> GetByIdAsync(Guid id);
    Task<Contador_UtilizadorDto> CreateAsync(Contador_UtilizadorCreateDto createDto);
    Task<Contador_UtilizadorDto?> UpdateAsync(Guid id, Contador_UtilizadorUpdateDto updateDto);
    Task<bool> DeleteAsync(Guid id);
}

public class Contador_UtilizadorService : IContador_UtilizadorService
{
    private readonly IContador_UtilizadorRepository _repository;
    private readonly IContadorRepository _contadorRepository;
    private readonly IUtilizadorRepository _utilizadorRepository;
    private readonly IMapper _mapper;
    private readonly ILogger<Contador_UtilizadorService> _logger;

    public Contador_UtilizadorService(
        IContador_UtilizadorRepository repository,
        IContadorRepository contadorRepository,
        IUtilizadorRepository utilizadorRepository,
        IMapper mapper,
        ILogger<Contador_UtilizadorService> logger)
    {
        _repository = repository;
        _contadorRepository = contadorRepository;
        _utilizadorRepository = utilizadorRepository;
        _mapper = mapper;
        _logger = logger;
    }

    public async Task<IEnumerable<Contador_UtilizadorDto>> GetAllAsync()
    {
        try
        {
            var associations = await _repository.GetAllAsync();
            return _mapper.Map<IEnumerable<Contador_UtilizadorDto>>(associations);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter todas as associações");
            throw;
        }
    }

    public async Task<Contador_UtilizadorDto?> GetByIdAsync(Guid id)
    {
        try
        {
            var association = await _repository.GetByIdAsync(id);
            return association == null ? null : _mapper.Map<Contador_UtilizadorDto>(association);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter associação por ID: {Id}", id);
            throw;
        }
    }

    public async Task<Contador_UtilizadorDto> CreateAsync(Contador_UtilizadorCreateDto createDto)
    {
        try
        {
            // Validar se contador existe
            var contadorExists = await _contadorRepository.ExistsAsync(createDto.ContadorId);
            if (!contadorExists)
            {
                throw new InvalidOperationException("Contador não encontrado");
            }

            // Validar se utilizador existe
            var utilizadorExists = await _utilizadorRepository.ExistsAsync(createDto.UtilizadorId);
            if (!utilizadorExists)
            {
                throw new InvalidOperationException("Utilizador não encontrado");
            }

            var association = _mapper.Map<Contador_Utilizador>(createDto);
            association.Id = Guid.NewGuid();
            association.CriadoEm = DateTime.UtcNow;
            
            var created = await _repository.CreateAsync(association);
            var result = await _repository.GetByIdAsync(created.Id);
            return _mapper.Map<Contador_UtilizadorDto>(result);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao criar associação");
            throw;
        }
    }

    public async Task<Contador_UtilizadorDto?> UpdateAsync(Guid id, Contador_UtilizadorUpdateDto updateDto)
    {
        try
        {
            var association = await _repository.GetByIdAsync(id);
            if (association == null)
            {
                return null;
            }

            _mapper.Map(updateDto, association);
            
            var updated = await _repository.UpdateAsync(association);
            return _mapper.Map<Contador_UtilizadorDto>(updated);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao atualizar associação: {Id}", id);
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
            _logger.LogError(ex, "Erro ao excluir associação: {Id}", id);
            throw;
        }
    }
}
