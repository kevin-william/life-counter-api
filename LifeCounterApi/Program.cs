using LifeCounterApi.Data;
using LifeCounterApi.Data.Repositories;
using LifeCounterApi.Services;
using Microsoft.EntityFrameworkCore;
using System.Reflection;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.

// Configure Database
builder.Services.AddDbContext<ApplicationDbContext>(options =>
    options.UseSqlServer(builder.Configuration.GetConnectionString("DefaultConnection")));

// Configure AutoMapper
builder.Services.AddAutoMapper(Assembly.GetExecutingAssembly());

// Register Repositories
builder.Services.AddScoped<IContadorRepository, ContadorRepository>();
builder.Services.AddScoped<IUtilizadorRepository, UtilizadorRepository>();
builder.Services.AddScoped<IContador_UtilizadorRepository, Contador_UtilizadorRepository>();

// Register Services
builder.Services.AddScoped<IContadorService, ContadorService>();
builder.Services.AddScoped<IUtilizadorService, UtilizadorService>();
builder.Services.AddScoped<IContador_UtilizadorService, Contador_UtilizadorService>();

// Configure Controllers
builder.Services.AddControllers()
    .ConfigureApiBehaviorOptions(options =>
    {
        options.SuppressModelStateInvalidFilter = false;
    });

// Configure Swagger/OpenAPI
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

// Configure CORS
builder.Services.AddCors(options =>
{
    options.AddPolicy("AllowAll",
        policy =>
        {
            policy.AllowAnyOrigin()
                  .AllowAnyMethod()
                  .AllowAnyHeader();
        });
});

// Configure Logging
builder.Logging.ClearProviders();
builder.Logging.AddConsole();
builder.Logging.AddDebug();

var app = builder.Build();

// Configure the HTTP request pipeline.
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI(c =>
    {
        c.SwaggerEndpoint("/swagger/v1/swagger.json", "Life Counter API v1");
        c.RoutePrefix = string.Empty; // Swagger at root
    });
}

app.UseHttpsRedirection();

app.UseCors("AllowAll");

app.UseAuthorization();

app.MapControllers();

app.Run();
