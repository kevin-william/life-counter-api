using LifeCounterApi.Models.DTOs;
using LifeCounterApi.Services;
using Microsoft.AspNetCore.Mvc;

namespace LifeCounterApi.Controllers;

[ApiController]
[Route("api/[controller]")]
[Produces("application/json")]
public class ContadoresController : ControllerBase
{
    private readonly IContadorService _service;
    private readonly ILogger<ContadoresController> _logger;

    public ContadoresController(IContadorService service, ILogger<ContadoresController> logger)
    {
        _service = service;
        _logger = logger;
    }

    /// <summary>
    /// Obtém todos os contadores
    /// </summary>
    [HttpGet]
    [ProducesResponseType(typeof(IEnumerable<ContadorDto>), StatusCodes.Status200OK)]
    public async Task<ActionResult<IEnumerable<ContadorDto>>> GetAll()
    {
        try
        {
            var contadores = await _service.GetAllAsync();
            return Ok(contadores);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter contadores");
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Obtém um contador por ID
    /// </summary>
    [HttpGet("{id}")]
    [ProducesResponseType(typeof(ContadorDto), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    public async Task<ActionResult<ContadorDto>> GetById(Guid id)
    {
        try
        {
            var contador = await _service.GetByIdAsync(id);
            if (contador == null)
            {
                return NotFound(new { message = "Contador não encontrado" });
            }
            return Ok(contador);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter contador: {Id}", id);
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Obtém utilizadores associados a um contador
    /// </summary>
    [HttpGet("{id}/utilizadores")]
    [ProducesResponseType(typeof(IEnumerable<UtilizadorDto>), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    public async Task<ActionResult<IEnumerable<UtilizadorDto>>> GetUtilizadores(Guid id)
    {
        try
        {
            var contador = await _service.GetByIdAsync(id);
            if (contador == null)
            {
                return NotFound(new { message = "Contador não encontrado" });
            }

            var utilizadores = await _service.GetUtilizadoresAsync(id);
            return Ok(utilizadores);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao obter utilizadores do contador: {Id}", id);
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Cria um novo contador
    /// </summary>
    [HttpPost]
    [ProducesResponseType(typeof(ContadorDto), StatusCodes.Status201Created)]
    [ProducesResponseType(StatusCodes.Status400BadRequest)]
    public async Task<ActionResult<ContadorDto>> Create([FromBody] ContadorCreateDto createDto)
    {
        try
        {
            if (!ModelState.IsValid)
            {
                return BadRequest(ModelState);
            }

            var contador = await _service.CreateAsync(createDto);
            return CreatedAtAction(nameof(GetById), new { id = contador.Id }, contador);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao criar contador");
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Atualiza um contador existente
    /// </summary>
    [HttpPut("{id}")]
    [ProducesResponseType(typeof(ContadorDto), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    [ProducesResponseType(StatusCodes.Status400BadRequest)]
    public async Task<ActionResult<ContadorDto>> Update(Guid id, [FromBody] ContadorUpdateDto updateDto)
    {
        try
        {
            if (!ModelState.IsValid)
            {
                return BadRequest(ModelState);
            }

            var contador = await _service.UpdateAsync(id, updateDto);
            if (contador == null)
            {
                return NotFound(new { message = "Contador não encontrado" });
            }
            return Ok(contador);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao atualizar contador: {Id}", id);
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Incrementa o valor de um contador
    /// </summary>
    [HttpPatch("{id}/incrementar")]
    [ProducesResponseType(typeof(ContadorDto), StatusCodes.Status200OK)]
    [ProducesResponseType(StatusCodes.Status404NotFound)]
    public async Task<ActionResult<ContadorDto>> Incrementar(Guid id, [FromBody] ContadorIncrementarDto incrementarDto)
    {
        try
        {
            var contador = await _service.IncrementarAsync(id, incrementarDto.Incremento);
            if (contador == null)
            {
                return NotFound(new { message = "Contador não encontrado" });
            }
            return Ok(contador);
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao incrementar contador: {Id}", id);
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }

    /// <summary>
    /// Exclui um contador
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
                return NotFound(new { message = "Contador não encontrado" });
            }
            return NoContent();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Erro ao excluir contador: {Id}", id);
            return StatusCode(500, new { message = "Erro interno do servidor" });
        }
    }
}
