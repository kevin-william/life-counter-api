# Life Counter API - Rust Edition

Uma API Rust com Actix-web para persistência e obtenção de dados da Life Counter App.

## Descrição

API RESTful desenvolvida em Rust com Actix-web, Diesel ORM e PostgreSQL para gerenciamento de contadores e utilizadores.

## Características

- ✅ Rust com Actix-web 4.9
- ✅ Diesel ORM com PostgreSQL
- ✅ Migrações automáticas ao iniciar
- ✅ Validações de dados com Validator
- ✅ Logging estruturado
- ✅ Tratamento de erros
- ✅ CORS configurado
- ✅ Serialização JSON com Serde

## Entidades

### Contador
- `id` (UUID) - Chave primária UUID
- `nome` (String) - Nome do contador
- `valor` (i32) - Valor atual do contador
- `criado_em` (DateTime) - Data de criação
- `atualizado_em` (DateTime) - Data de última atualização

### Utilizador
- `id` (UUID) - Chave primária UUID
- `nome` (String) - Nome do utilizador
- `email` (String) - Email único do utilizador
- `criado_em` (DateTime) - Data de criação
- `atualizado_em` (DateTime) - Data de última atualização

### Contador_Utilizador
- `id` (UUID) - Chave primária UUID
- `contador_id` (UUID) - Referência ao contador
- `utilizador_id` (UUID) - Referência ao utilizador
- `observacoes` (String) - Observações opcionais
- `criado_em` (DateTime) - Data de criação

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

### Health Check & Logs

- `GET /health` - Health check básico (status, versão, timestamp)
- `GET /health/detailed` - Health check detalhado (inclui status do banco de dados)
- `GET /health/logs` - Visualiza logs recentes da aplicação
  - Query param `limit`: número de logs a retornar (default: 100, max: 1000)
  - Exemplo: `/health/logs?limit=50`

## Configuração

### Pré-requisitos

- Rust (1.70+)
- Cargo
- PostgreSQL
- Diesel CLI (para migrações)

### Instalação do Diesel CLI

```bash
cargo install diesel_cli --no-default-features --features postgres
```

### Variáveis de Ambiente

Crie um arquivo `.env` na raiz do projeto:

```env
DATABASE_URL=postgres://postgres:postgres@localhost/LifeCounterDb
HOST=0.0.0.0
PORT=8080
RUST_LOG=info
```

**Notas importantes:**
- As tabelas do banco de dados são criadas automaticamente na primeira execução da aplicação através de migrações.
- ⚠️ **Segurança**: Os valores de exemplo acima usam credenciais padrão. Para ambientes de produção, use variáveis de ambiente seguras. Nunca commite senhas reais no controle de versão.

### Executar a API

```bash
cd life-counter-api-rust

# Instalar dependências e compilar
cargo build

# Executar em modo de desenvolvimento
cargo run

# Executar em modo de produção
cargo build --release
./target/release/life_counter_api
```

A API estará disponível em:
- HTTP: `http://localhost:8080`

### Migrações (Diesel)

As migrações são aplicadas automaticamente ao iniciar a aplicação. Se precisar executar manualmente:

```bash
# Aplicar migrações
diesel migration run

# Reverter última migração
diesel migration revert

# Criar nova migração
diesel migration generate <nome_da_migracao>
```

## Estrutura do Projeto

```
life-counter-api-rust/
├── migrations/            # Migrações do banco de dados
├── src/
│   ├── db/               # Configuração do banco de dados e schema
│   ├── dto/              # Data Transfer Objects
│   ├── handlers/         # Handlers HTTP (Controllers)
│   ├── models/           # Entidades do domínio
│   ├── services/         # Lógica de negócio
│   ├── lib.rs            # Biblioteca para testes
│   └── main.rs           # Ponto de entrada da aplicação
├── tests/                # Testes de integração
├── Cargo.toml            # Dependências do projeto
└── .env                  # Variáveis de ambiente (não versionar)
```

## Tecnologias Utilizadas

- **Rust** - Linguagem de programação
- **Actix-web 4.9** - Framework web async
- **Diesel 2.2** - ORM e query builder
- **PostgreSQL** - Banco de dados
- **Serde 1.0** - Serialização/Deserialização
- **UUID 1.11** - Geração de UUIDs
- **Chrono 0.4** - Manipulação de datas
- **Validator 0.18** - Validação de dados
- **env_logger 0.11** - Sistema de logging

## Testes

A API possui uma suite de testes abrangente cobrindo todas as camadas da aplicação:

### Tipos de Testes

**Testes Unitários (24 testes)**
- Validação de DTOs (Contador, Utilizador, ContadorUtilizador)
- Validação de campos obrigatórios
- Validação de comprimento de strings
- Validação de formato de email
- Serialização/deserialização JSON

**Testes de Integração**
- Testes de endpoints da API (GET, POST, PUT, PATCH, DELETE)
- Testes de relacionamentos entre entidades
- Testes de validação de entrada
- Testes de casos de erro (404, 400)
- Testes de CRUD completo

### Executar Testes

```bash
# Executar todos os testes (unitários + integração)
cargo test

# Executar apenas testes unitários
cargo test --lib

# Executar apenas testes de integração
cargo test --test '*'

# Executar testes com output detalhado
cargo test -- --nocapture

# Executar um teste específico
cargo test test_contadores_create_and_get

# Executar testes com logging
RUST_LOG=debug cargo test -- --nocapture
```

### Cobertura de Testes

✅ **DTOs (100%)** - Todos os DTOs têm testes de validação  
✅ **Endpoints (100%)** - Todos os 19 endpoints testados  
✅ **CRUD Completo** - Create, Read, Update, Delete  
✅ **Validações** - Campos obrigatórios, limites, formatos  
✅ **Relacionamentos** - Associações entre entidades  
✅ **Casos de Erro** - 400, 404, validações

**Nota**: Os testes de integração requerem acesso ao banco de dados PostgreSQL configurado no `.env`.

## Validações

A API implementa validações em múltiplos níveis:

- **Validação de modelo** - Atributos de validação nos DTOs usando Validator
- **Validação de negócio** - Lógica customizada nos serviços
- **Validação de integridade** - Constraints e índices no banco de dados

## Tratamento de Erros

- **400 Bad Request** - Dados inválidos ou erros de validação
- **404 Not Found** - Recurso não encontrado
- **500 Internal Server Error** - Erros do servidor com logging

## Logging

A API utiliza o `env_logger` para logging estruturado:

- Logs de informação para operações bem-sucedidas
- Logs de erro para exceções
- Configurável através da variável `RUST_LOG`

## Desenvolvimento

### Hot Reload com Cargo Watch

Para desenvolvimento com recarga automática:

```bash
# Instalar cargo-watch
cargo install cargo-watch

# Executar com hot reload
cargo watch -x run
```

## Diferenças da Implementação .NET

Esta implementação Rust mantém a mesma funcionalidade e API da versão .NET original, mas com as seguintes características específicas:

- **Performance**: Rust oferece performance superior e menor uso de memória
- **Segurança**: Sistema de tipos do Rust previne muitos bugs em tempo de compilação
- **Concorrência**: Actix-web oferece excelente performance assíncrona
- **Deployment**: Binário standalone sem necessidade de runtime

## Licença

Este projeto está sob a licença MIT.
