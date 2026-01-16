using System.ComponentModel.DataAnnotations;

namespace LifeCounterApi.Models.DTOs;

public class ContadorDto
{
    public Guid Id { get; set; }
    public string Nome { get; set; } = string.Empty;
    public int Valor { get; set; }
    public DateTime CriadoEm { get; set; }
    public DateTime AtualizadoEm { get; set; }
}

public class ContadorCreateDto
{
    [Required(ErrorMessage = "O nome é obrigatório")]
    [MaxLength(200, ErrorMessage = "O nome não pode ter mais de 200 caracteres")]
    public string Nome { get; set; } = string.Empty;
    
    public int Valor { get; set; } = 0;
}

public class ContadorUpdateDto
{
    [MaxLength(200, ErrorMessage = "O nome não pode ter mais de 200 caracteres")]
    public string? Nome { get; set; }
    
    public int? Valor { get; set; }
}

public class ContadorIncrementarDto
{
    public int Incremento { get; set; } = 1;
}
