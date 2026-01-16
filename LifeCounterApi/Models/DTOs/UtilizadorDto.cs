using System.ComponentModel.DataAnnotations;

namespace LifeCounterApi.Models.DTOs;

public class UtilizadorDto
{
    public Guid Id { get; set; }
    public string Nome { get; set; } = string.Empty;
    public string Email { get; set; } = string.Empty;
    public DateTime CriadoEm { get; set; }
    public DateTime AtualizadoEm { get; set; }
}

public class UtilizadorCreateDto
{
    [Required(ErrorMessage = "O nome é obrigatório")]
    [MaxLength(200, ErrorMessage = "O nome não pode ter mais de 200 caracteres")]
    public string Nome { get; set; } = string.Empty;
    
    [Required(ErrorMessage = "O email é obrigatório")]
    [EmailAddress(ErrorMessage = "Email inválido")]
    [MaxLength(200, ErrorMessage = "O email não pode ter mais de 200 caracteres")]
    public string Email { get; set; } = string.Empty;
}

public class UtilizadorUpdateDto
{
    [MaxLength(200, ErrorMessage = "O nome não pode ter mais de 200 caracteres")]
    public string? Nome { get; set; }
    
    [EmailAddress(ErrorMessage = "Email inválido")]
    [MaxLength(200, ErrorMessage = "O email não pode ter mais de 200 caracteres")]
    public string? Email { get; set; }
}
