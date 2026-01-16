using LifeCounterApi.Models.Entities;
using Microsoft.EntityFrameworkCore;

namespace LifeCounterApi.Data;

public class ApplicationDbContext : DbContext
{
    public ApplicationDbContext(DbContextOptions<ApplicationDbContext> options) : base(options)
    {
    }
    
    public DbSet<Contador> Contadores { get; set; }
    public DbSet<Utilizador> Utilizadores { get; set; }
    public DbSet<Contador_Utilizador> Contador_Utilizadores { get; set; }
    
    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        base.OnModelCreating(modelBuilder);
        
        // Configuração de índices
        modelBuilder.Entity<Contador>()
            .HasIndex(c => c.Nome);
        
        modelBuilder.Entity<Utilizador>()
            .HasIndex(u => u.Email)
            .IsUnique();
        
        modelBuilder.Entity<Contador_Utilizador>()
            .HasIndex(cu => new { cu.ContadorId, cu.UtilizadorId })
            .IsUnique();
        
        // Configuração de relacionamentos
        modelBuilder.Entity<Contador_Utilizador>()
            .HasOne(cu => cu.Contador)
            .WithMany(c => c.ContadorUtilizadores)
            .HasForeignKey(cu => cu.ContadorId)
            .OnDelete(DeleteBehavior.Cascade);
        
        modelBuilder.Entity<Contador_Utilizador>()
            .HasOne(cu => cu.Utilizador)
            .WithMany(u => u.ContadorUtilizadores)
            .HasForeignKey(cu => cu.UtilizadorId)
            .OnDelete(DeleteBehavior.Cascade);
    }
}
