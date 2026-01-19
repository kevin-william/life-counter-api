# life-counter-api

Uma API para persistência e obtenção de dados da life counter app.

Este repositório contém duas implementações da mesma API:
- **[.NET 8 API](./LifeCounterApi)** - Implementação original em C# com ASP.NET Core
- **[Rust API](./life-counter-api-rust)** - Implementação em Rust com Actix-web

## Implementações Disponíveis

### .NET 8 API (`/LifeCounterApi`)

API RESTful desenvolvida em .NET 8 com Entity Framework Core, AutoMapper e Swagger para gerenciamento de contadores e utilizadores.

### Rust API (`/life-counter-api-rust`)

Implementação completa da mesma API usando Rust com Actix-web e Diesel ORM. Oferece:
- Performance superior e menor uso de memória
- Segurança garantida pelo sistema de tipos do Rust
- Binário standalone sem necessidade de runtime
- Excelente suporte para programação assíncrona

## Escolhendo uma Implementação

Ambas as implementações oferecem a mesma API RESTful e podem ser usadas de forma intercambiável:

**Use a implementação .NET se:**
- Você já tem expertise em C# e .NET
- Quer usar o ecossistema .NET (Visual Studio, Azure, etc.)
- Precisa de documentação automática com Swagger

**Use a implementação Rust se:**
- Precisa de máxima performance e eficiência de memória
- Quer garantias de segurança em tempo de compilação
- Prefere um binário standalone sem dependências de runtime
- Está desenvolvendo em ambientes com recursos limitados

## Descrição

API RESTful desenvolvida em .NET 8 com Entity Framework Core, AutoMapper e Swagger para gerenciamento de contadores e utilizadores.

## Características (.NET)

- ✅ .NET 8 Web API
- ✅ Entity Framework Core com PostgreSQL
- ✅ Criação automática de tabelas ao iniciar
- ✅ Repository Pattern
- ✅ Dependency Injection
- ✅ AutoMapper para mapeamento de DTOs
- ✅ Swagger/OpenAPI para documentação
- ✅ Validações de dados
- ✅ Logging estruturado
- ✅ Tratamento de erros
- ✅ CORS configurado

## Entidades

### Contador
- `Id` (Guid) - Chave primária UUID
- `Nome` (string) - Nome do contador
- `Valor` (int) - Valor atual do contador
- `CriadoEm` (DateTime) - Data de criação
- `AtualizadoEm` (DateTime) - Data de última atualização

### Utilizador
- `Id` (Guid) - Chave primária UUID
- `Nome` (string) - Nome do utilizador
- `Email` (string) - Email único do utilizador
- `CriadoEm` (DateTime) - Data de criação
- `AtualizadoEm` (DateTime) - Data de última atualização

### Contador_Utilizador
- `Id` (Guid) - Chave primária UUID
- `ContadorId` (Guid) - Referência ao contador
- `UtilizadorId` (Guid) - Referência ao utilizador
- `Observacoes` (string) - Observações opcionais
- `CriadoEm` (DateTime) - Data de criação

## Endpoints

### Contadores

- `GET /api/contadores` - Lista todos os contadores
- `GET /api/contadores/{id}` - Obtém um contador por ID
- `GET /api/contadores/{id}/utilizadores` - Lista utilizadores associados a um contador
- `POST /api/contadores` - Cria um novo contador
- `PUT /api/contadores/{id}` - Atualiza um contador
- `PATCH /api/contadores/{id}/incrementar` - Incrementa o valor de um contador
- `DELETE /api/contadores/{id}` - Remove um contador

### Utilizadores

- `GET /api/utilizadores` - Lista todos os utilizadores
- `GET /api/utilizadores/{id}` - Obtém um utilizador por ID
- `GET /api/utilizadores/{id}/contadores` - Lista contadores associados a um utilizador
- `POST /api/utilizadores` - Cria um novo utilizador
- `PUT /api/utilizadores/{id}` - Atualiza um utilizador
- `DELETE /api/utilizadores/{id}` - Remove um utilizador

### Contador_Utilizadores

- `GET /api/contador_utilizadores` - Lista todas as associações
- `GET /api/contador_utilizadores/{id}` - Obtém uma associação por ID
- `POST /api/contador_utilizadores` - Cria uma nova associação
- `PUT /api/contador_utilizadores/{id}` - Atualiza uma associação
- `DELETE /api/contador_utilizadores/{id}` - Remove uma associação

## Configuração

### Pré-requisitos

- .NET 8 SDK
- PostgreSQL

### String de Conexão

Atualize a string de conexão em `appsettings.json` ou `appsettings.Development.json`:

```json
{
  "ConnectionStrings": {
    "DefaultConnection": "Host=localhost;Database=LifeCounterDb;Username=postgres;Password=postgres"
  }
}
```

**Notas importantes:**
- As tabelas do banco de dados são criadas automaticamente na primeira execução da aplicação se não existirem.
- ⚠️ **Segurança**: Os valores de exemplo acima usam credenciais padrão. Para ambientes de produção, use variáveis de ambiente ou o User Secrets do .NET para armazenar credenciais sensíveis. Nunca commite senhas reais no controle de versão.

### Executar a API

```bash
cd LifeCounterApi
dotnet restore
dotnet build
dotnet run
```

A API estará disponível em:
- HTTP: `http://localhost:5142`
- HTTPS: `https://localhost:7212`
- Swagger UI: `http://localhost:5142` ou `https://localhost:7212`

### Migrations (Entity Framework)

**Nota:** A aplicação cria automaticamente as tabelas necessárias ao iniciar se elas não existirem. No entanto, se preferir usar migrations do Entity Framework:

```bash
# Criar uma migration
dotnet ef migrations add InitialCreate

# Aplicar migrations ao banco de dados
dotnet ef database update
```

## Estrutura do Projeto

```
LifeCounterApi/
├── Controllers/           # Controladores da API
├── Data/
│   ├── ApplicationDbContext.cs
│   └── Repositories/      # Implementação do padrão Repository
├── Mappings/              # Perfis do AutoMapper
├── Models/
│   ├── DTOs/             # Data Transfer Objects
│   └── Entities/         # Entidades do domínio
├── Services/             # Lógica de negócio
└── Program.cs            # Configuração da aplicação
```

## Tecnologias Utilizadas

- **ASP.NET Core 8.0** - Framework web
- **Entity Framework Core 8.0** - ORM
- **PostgreSQL** - Banco de dados
- **Npgsql** - Driver PostgreSQL para .NET
- **AutoMapper 12.0** - Mapeamento objeto-objeto
- **Swashbuckle.AspNetCore 10.1** - Documentação Swagger/OpenAPI

## Validações

A API implementa validações em múltiplos níveis:

- **Validação de modelo** - Atributos de validação nos DTOs
- **Validação de negócio** - Lógica customizada nos serviços
- **Validação de integridade** - Constraints e índices no banco de dados

## Tratamento de Erros

- **400 Bad Request** - Dados inválidos ou erros de validação
- **404 Not Found** - Recurso não encontrado
- **500 Internal Server Error** - Erros do servidor com logging

## Logging

A API utiliza o sistema de logging integrado do ASP.NET Core:

- Logs de informação para operações bem-sucedidas
- Logs de erro para exceções
- Logs de aviso para validações de negócio

## Desenvolvimento

### Adicionar uma Nova Entidade

1. Criar a entidade em `Models/Entities/`
2. Criar os DTOs em `Models/DTOs/`
3. Atualizar o `ApplicationDbContext`
4. Criar interface e implementação do repository em `Data/Repositories/`
5. Criar interface e implementação do service em `Services/`
6. Atualizar `MappingProfile` com os mapeamentos
7. Criar o controller em `Controllers/`
8. Registrar dependências em `Program.cs`

## Quick Start

### Executar a API .NET

```bash
cd LifeCounterApi
dotnet restore
dotnet build
dotnet run
```

A API estará disponível em `http://localhost:5142` com Swagger UI.

### Executar a API Rust

```bash
cd life-counter-api-rust
cargo build
cargo run
```

A API estará disponível em `http://localhost:8080`.

Para mais detalhes sobre cada implementação, consulte o README específico de cada pasta.

## Licença

Este projeto está sob a licença MIT.
