-- Create Utilizadores table
CREATE TABLE utilizadores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nome VARCHAR(200) NOT NULL,
    email VARCHAR(200) NOT NULL UNIQUE,
    criado_em TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    atualizado_em TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create Contadores table
CREATE TABLE contadores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nome VARCHAR(200) NOT NULL,
    valor INTEGER NOT NULL DEFAULT 0,
    criado_em TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    atualizado_em TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create Contador_Utilizadores junction table
CREATE TABLE contador_utilizadores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contador_id UUID NOT NULL REFERENCES contadores(id) ON DELETE CASCADE,
    utilizador_id UUID NOT NULL REFERENCES utilizadores(id) ON DELETE CASCADE,
    observacoes VARCHAR(500),
    criado_em TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for foreign keys
CREATE INDEX idx_contador_utilizadores_contador_id ON contador_utilizadores(contador_id);
CREATE INDEX idx_contador_utilizadores_utilizador_id ON contador_utilizadores(utilizador_id);
