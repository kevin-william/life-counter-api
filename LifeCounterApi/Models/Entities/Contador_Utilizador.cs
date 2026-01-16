using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace LifeCounterApi.Models.Entities;

public class Contador_Utilizador
{
    [Key]
    public Guid Id { get; set; } = Guid.NewGuid();
    
    [Required]
    public Guid ContadorId { get; set; }
    
    [Required]
    public Guid UtilizadorId { get; set; }
    
    [MaxLength(500)]
    public string? Observacoes { get; set; }
    
    public DateTime CriadoEm { get; set; } = DateTime.UtcNow;
    
    // Navigation properties
    [ForeignKey(nameof(ContadorId))]
    public virtual Contador Contador { get; set; } = null!;
    
    [ForeignKey(nameof(UtilizadorId))]
    public virtual Utilizador Utilizador { get; set; } = null!;
}
