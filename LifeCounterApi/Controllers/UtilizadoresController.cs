using LifeCounterApi.Models.DTOs;
using LifeCounterApi.Services;
using Microsoft.AspNetCore.Mvc;

namespace LifeCounterApi.Controllers;

[ApiController]
[Route("api/[controller]")]
[Produces("application/json")]
public class UtilizadoresController : ControllerBase
{
    private readonly IUtilizadorService _service;
    private readonly ILogger<UtilizadoresController> _logger;

    public UtilizadoresController(IUtilizadorService service, ILogger<UtilizadoresController> logger)
    {
        _service = service;
        _logger = logger;
    }

    /// <summary>
    /// Obtém todos os utilizadores
    /// </summary>
    [HttpGet]
    [ProducesResponseType(typeof(IEnumerable<UtilizadorDto>), StatusCodes.Status200OK)]
    public async Task<ActionResult<IEnumerable<UtilizadorDto>>> GetAll()
    {
        try
        {
            var utilizadores = await _service.GetAllAsync();
            return Ok(utilizadores);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter utilizadores");
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Obtém um utilizador por ID
    /// </summary>
    [HttpGet("{id}")]
    [ProducesResponseType(typeof(UtilizadorDto), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    public async Task<ActionResult<UtilizadorDto>> GetById(Guid id)
    {
        try
        {
            var utilizador = await _service.GetByIdAsync(id);
            if (utilizador == null)
            {
                return NotFound(new { message = "Utilizador não encontrado" });
            }
            return Ok(utilizador);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter utilizador: {Id}", id);
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Obtém contadores associados a um utilizador
    /// </summary>
    [HttpGet("{id}/contadores")]
    [ProducesResponseType(typeof(IEnumerable<ContadorDto>), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    public async Task<ActionResult<IEnumerable<ContadorDto>>> GetContadores(Guid id)
    {
        try
        {
            var utilizador = await _service.GetByIdAsync(id);
            if (utilizador == null)
            {
                return NotFound(new { message = "Utilizador não encontrado" });
            }

            var contadores = await _service.GetContadoresAsync(id);
            return Ok(contadores);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter contadores do utilizador: {Id}", id);
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Cria um novo utilizador
    /// </summary>
    [HttpPost]
    [ProducesResponseType(typeof(UtilizadorDto), StatusCodes.Status201Created)]
    [ProducesResponseType(StatusCodes.Status400BadRequest)]
    public async Task<ActionResult<UtilizadorDto>> Create([FromBody] UtilizadorCreateDto createDto)
    {
        try
        {
            if (!ModelState.IsValid)
            {
                return BadRequest(ModelState);
            }

            var utilizador = await _service.CreateAsync(createDto);
            return CreatedAtAction(nameof(GetById), new { id = utilizador.Id }, utilizador);
        }
        catch (InvalidOperationException ex)
        {
            _logger.LogWarning(ex, "Erro de validação ao criar utilizador");
            return BadRequest(new { message = ex.Message });
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao criar utilizador");
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Atualiza um utilizador existente
    /// </summary>
    [HttpPut("{id}")]
    [ProducesResponseType(typeof(UtilizadorDto), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    [ProducesResponseType(StatusCodes.Status400BadRequest)]
    public async Task<ActionResult<UtilizadorDto>> Update(Guid id, [FromBody] UtilizadorUpdateDto updateDto)
    {
        try
        {
            if (!ModelState.IsValid)
            {
                return BadRequest(ModelState);
            }

            var utilizador = await _service.UpdateAsync(id, updateDto);
            if (utilizador == null)
            {
                return NotFound(new { message = "Utilizador não encontrado" });
            }
            return Ok(utilizador);
        }
        catch (InvalidOperationException ex)
        {
            _logger.LogWarning(ex, "Erro de validação ao atualizar utilizador");
            return BadRequest(new { message = ex.Message });
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao atualizar utilizador: {Id}", id);
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Exclui um utilizador
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
                return NotFound(new { message = "Utilizador não encontrado" });
            }
            return NoContent();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao excluir utilizador: {Id}", id);
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }
}
