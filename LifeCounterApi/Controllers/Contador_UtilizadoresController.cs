using LifeCounterApi.Models.DTOs;
using LifeCounterApi.Services;
using Microsoft.AspNetCore.Mvc;

namespace LifeCounterApi.Controllers;

[ApiController]
[Route("api/[controller]")]
[Produces("application/json")]
public class Contador_UtilizadoresController : ControllerBase
{
    private readonly IContador_UtilizadorService _service;
    private readonly ILogger<Contador_UtilizadoresController> _logger;

    public Contador_UtilizadoresController(IContador_UtilizadorService service, ILogger<Contador_UtilizadoresController> logger)
    {
        _service = service;
        _logger = logger;
    }

    /// <summary>
    /// Obtém todas as associações entre contadores e utilizadores
    /// </summary>
    [HttpGet]
    [ProducesResponseType(typeof(IEnumerable<Contador_UtilizadorDto>), StatusCodes.Status200OK)]
    public async Task<ActionResult<IEnumerable<Contador_UtilizadorDto>>> GetAll()
    {
        try
        {
            var associations = await _service.GetAllAsync();
            return Ok(associations);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter associações");
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Obtém uma associação por ID
    /// </summary>
    [HttpGet("{id}")]
    [ProducesResponseType(typeof(Contador_UtilizadorDto), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    public async Task<ActionResult<Contador_UtilizadorDto>> GetById(Guid id)
    {
        try
        {
            var association = await _service.GetByIdAsync(id);
            if (association == null)
            {
                return NotFound(new { message = "Associação não encontrada" });
            }
            return Ok(association);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter associação: {Id}", id);
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Cria uma nova associação entre contador e utilizador
    /// </summary>
    [HttpPost]
    [ProducesResponseType(typeof(Contador_UtilizadorDto), StatusCodes.Status201Created)]
    [ProducesResponseType(StatusCodes.Status400BadRequest)]
    public async Task<ActionResult<Contador_UtilizadorDto>> Create([FromBody] Contador_UtilizadorCreateDto createDto)
    {
        try
        {
            if (!ModelState.IsValid)
            {
                return BadRequest(ModelState);
            }

            var association = await _service.CreateAsync(createDto);
            return CreatedAtAction(nameof(GetById), new { id = association.Id }, association);
        }
        catch (InvalidOperationException ex)
        {
            _logger.LogWarning(ex, "Erro de validação ao criar associação");
            return BadRequest(new { message = ex.Message });
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao criar associação");
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Atualiza uma associação existente
    /// </summary>
    [HttpPut("{id}")]
    [ProducesResponseType(typeof(Contador_UtilizadorDto), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    [ProducesResponseType(StatusCodes.Status400BadRequest)]
    public async Task<ActionResult<Contador_UtilizadorDto>> Update(Guid id, [FromBody] Contador_UtilizadorUpdateDto updateDto)
    {
        try
        {
            if (!ModelState.IsValid)
            {
                return BadRequest(ModelState);
            }

            var association = await _service.UpdateAsync(id, updateDto);
            if (association == null)
            {
                return NotFound(new { message = "Associação não encontrada" });
            }
            return Ok(association);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao atualizar associação: {Id}", id);
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Exclui uma associação
    /// </summary>
    [HttpDelete("{id}")]
    [ProducesResponseType(StatusCodes.Status204NoContent)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    public async Task<IActionResult> Delete(Guid id)
    {
        try
        {
            var deleted = await _service.DeleteAsync(id);
            if (!deleted)
            {
                return NotFound(new { message = "Associação não encontrada" });
            }
            return NoContent();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao excluir associação: {Id}", id);
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }
}
