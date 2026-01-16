using System.ComponentModel.DataAnnotations;

namespace LifeCounterApi.Models.Entities;

public class Contador
{
    [Key]
    public Guid Id { get; set; } = Guid.NewGuid();
    
    [Required]
    [MaxLength(200)]
    public string Nome { get; set; } = string.Empty;
    
    public int Valor { get; set; } = 0;
    
    public DateTime CriadoEm { get; set; } = DateTime.UtcNow;
    
    public DateTime AtualizadoEm { get; set; } = DateTime.UtcNow;
    
    // Navigation property
    public virtual ICollection<Contador_Utilizador> ContadorUtilizadores { get; set; } = new List<Contador_Utilizador>();
}
