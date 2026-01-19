// @generated automatically by Diesel CLI.

diesel::table! {
    contadores (id) {
        id -> Uuid,
        nome -> Varchar,
        valor -> Int4,
        criado_em -> Timestamp,
        atualizado_em -> Timestamp,
    }
}

diesel::table! {
    contador_utilizadores (id) {
        id -> Uuid,
        contador_id -> Uuid,
        utilizador_id -> Uuid,
        observacoes -> Nullable<Varchar>,
        criado_em -> Timestamp,
    }
}

diesel::table! {
    utilizadores (id) {
        id -> Uuid,
        nome -> Varchar,
        email -> Varchar,
        criado_em -> Timestamp,
        atualizado_em -> Timestamp,
    }
}

diesel::joinable!(contador_utilizadores -> contadores (contador_id));
diesel::joinable!(contador_utilizadores -> utilizadores (utilizador_id));

diesel::allow_tables_to_appear_in_same_query!(
    contadores,
    contador_utilizadores,
    utilizadores,
);
