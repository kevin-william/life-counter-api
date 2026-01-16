using System.ComponentModel.DataAnnotations;

namespace LifeCounterApi.Models.DTOs;

public class Contador_UtilizadorDto
{
    public Guid Id { get; set; }
    public Guid ContadorId { get; set; }
    public Guid UtilizadorId { get; set; }
    public string? Observacoes { get; set; }
    public DateTime CriadoEm { get; set; }
    public ContadorDto? Contador { get; set; }
    public UtilizadorDto? Utilizador { get; set; }
}

public class Contador_UtilizadorCreateDto
{
    [Required(ErrorMessage = "O ID do contador é obrigatório")]
    public Guid ContadorId { get; set; }
    
    [Required(ErrorMessage = "O ID do utilizador é obrigatório")]
    public Guid UtilizadorId { get; set; }
    
    [MaxLength(500, ErrorMessage = "As observações não podem ter mais de 500 caracteres")]
    public string? Observacoes { get; set; }
}

public class Contador_UtilizadorUpdateDto
{
    [MaxLength(500, ErrorMessage = "As observações não podem ter mais de 500 caracteres")]
    public string? Observacoes { get; set; }
}
