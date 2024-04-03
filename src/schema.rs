// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ltree"))]
    pub struct Ltree;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "semver_triple"))]
    pub struct SemverTriple;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tsvector", schema = "pg_catalog"))]
    pub struct Tsvector;
}

diesel::table! {
    api_tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Bytea,
        name -> Varchar,
        created_at -> Timestamp,
        last_used_at -> Nullable<Timestamp>,
        revoked -> Bool,
        crate_scopes -> Nullable<Array<Nullable<Text>>>,
        endpoint_scopes -> Nullable<Array<Nullable<Text>>>,
        expired_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    background_jobs (id) {
        id -> Int8,
        job_type -> Text,
        data -> Jsonb,
        retries -> Int4,
        last_retry -> Timestamp,
        created_at -> Timestamp,
        priority -> Int2,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Ltree;

    categories (id) {
        id -> Int4,
        category -> Varchar,
        slug -> Varchar,
        description -> Varchar,
        crates_cnt -> Int4,
        created_at -> Timestamp,
        path -> Ltree,
    }
}

diesel::table! {
    crate_downloads (crate_id) {
        crate_id -> Int4,
        downloads -> Int8,
    }
}

diesel::table! {
    crate_infos (id) {
        id -> Int4,
        name -> Text,
        path -> Text,
        result -> Text,
    }
}

diesel::table! {
    crate_owner_invitations (invited_user_id, crate_id) {
        invited_user_id -> Int4,
        invited_by_user_id -> Int4,
        crate_id -> Int4,
        created_at -> Timestamp,
        token -> Text,
        token_generated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    crate_owners (crate_id, owner_id, owner_kind) {
        crate_id -> Int4,
        owner_id -> Int4,
        created_at -> Timestamp,
        created_by -> Nullable<Int4>,
        deleted -> Bool,
        updated_at -> Timestamp,
        owner_kind -> Int4,
        email_notifications -> Bool,
    }
}

diesel::table! {
    crate_results (id) {
        id -> Int4,
        compile_pass -> Bool,
        no_deadlock -> Bool,
        reason -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Tsvector;

    crates (id) {
        id -> Int4,
        name -> Varchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
        downloads -> Int4,
        description -> Nullable<Varchar>,
        homepage -> Nullable<Varchar>,
        documentation -> Nullable<Varchar>,
        readme -> Nullable<Varchar>,
        textsearchable_index_col -> Tsvector,
        repository -> Nullable<Varchar>,
        max_upload_size -> Nullable<Int4>,
        max_features -> Nullable<Int2>,
    }
}

diesel::table! {
    crates_categories (crate_id, category_id) {
        crate_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    crates_keywords (crate_id, keyword_id) {
        crate_id -> Int4,
        keyword_id -> Int4,
    }
}

diesel::table! {
    dependencies (id) {
        id -> Int4,
        version_id -> Int4,
        crate_id -> Int4,
        req -> Varchar,
        optional -> Bool,
        default_features -> Bool,
        features -> Array<Nullable<Text>>,
        target -> Nullable<Varchar>,
        kind -> Int4,
        explicit_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    emails (id) {
        id -> Int4,
        user_id -> Int4,
        email -> Varchar,
        verified -> Bool,
        token -> Text,
        token_generated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    follows (user_id, crate_id) {
        user_id -> Int4,
        crate_id -> Int4,
    }
}

diesel::table! {
    keywords (id) {
        id -> Int4,
        keyword -> Text,
        crates_cnt -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    metadata (total_downloads) {
        total_downloads -> Int8,
    }
}

diesel::table! {
    processed_log_files (path) {
        path -> Varchar,
        time -> Timestamptz,
    }
}

diesel::table! {
    publish_limit_buckets (user_id, action) {
        user_id -> Int4,
        tokens -> Int4,
        last_refill -> Timestamp,
        action -> Int4,
    }
}

diesel::table! {
    publish_rate_overrides (user_id, action) {
        user_id -> Int4,
        burst -> Int4,
        expires_at -> Nullable<Timestamp>,
        action -> Int4,
    }
}

diesel::table! {
    readme_renderings (version_id) {
        version_id -> Int4,
        rendered_at -> Timestamp,
    }
}

diesel::table! {
    reserved_crate_names (name) {
        name -> Text,
    }
}

diesel::table! {
    teams (id) {
        id -> Int4,
        login -> Varchar,
        github_id -> Int4,
        name -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        org_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        gh_access_token -> Varchar,
        gh_login -> Varchar,
        name -> Nullable<Varchar>,
        gh_avatar -> Nullable<Varchar>,
        gh_id -> Int4,
        account_lock_reason -> Nullable<Varchar>,
        account_lock_until -> Nullable<Timestamp>,
        is_admin -> Bool,
    }
}

diesel::table! {
    version_downloads (version_id, date) {
        version_id -> Int4,
        downloads -> Int4,
        counted -> Int4,
        date -> Date,
        processed -> Bool,
    }
}

diesel::table! {
    version_owner_actions (id) {
        id -> Int4,
        version_id -> Int4,
        user_id -> Int4,
        api_token_id -> Nullable<Int4>,
        action -> Int4,
        time -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SemverTriple;

    versions (id) {
        id -> Int4,
        crate_id -> Int4,
        num -> Varchar,
        updated_at -> Timestamp,
        created_at -> Timestamp,
        downloads -> Int4,
        features -> Jsonb,
        yanked -> Bool,
        license -> Nullable<Varchar>,
        crate_size -> Nullable<Int4>,
        published_by -> Nullable<Int4>,
        #[max_length = 64]
        checksum -> Bpchar,
        links -> Nullable<Varchar>,
        rust_version -> Nullable<Varchar>,
        semver_no_prerelease -> Nullable<SemverTriple>,
    }
}

diesel::table! {
    versions_published_by (version_id) {
        version_id -> Int4,
        email -> Varchar,
    }
}

diesel::joinable!(api_tokens -> users (user_id));
diesel::joinable!(crate_downloads -> crates (crate_id));
diesel::joinable!(crate_owner_invitations -> crates (crate_id));
diesel::joinable!(crate_owners -> crates (crate_id));
diesel::joinable!(crate_owners -> users (created_by));
diesel::joinable!(crates_categories -> categories (category_id));
diesel::joinable!(crates_categories -> crates (crate_id));
diesel::joinable!(crates_keywords -> crates (crate_id));
diesel::joinable!(crates_keywords -> keywords (keyword_id));
diesel::joinable!(dependencies -> crates (crate_id));
diesel::joinable!(dependencies -> versions (version_id));
diesel::joinable!(emails -> users (user_id));
diesel::joinable!(follows -> crates (crate_id));
diesel::joinable!(follows -> users (user_id));
diesel::joinable!(publish_limit_buckets -> users (user_id));
diesel::joinable!(publish_rate_overrides -> users (user_id));
diesel::joinable!(readme_renderings -> versions (version_id));
diesel::joinable!(version_downloads -> versions (version_id));
diesel::joinable!(version_owner_actions -> api_tokens (api_token_id));
diesel::joinable!(version_owner_actions -> users (user_id));
diesel::joinable!(version_owner_actions -> versions (version_id));
diesel::joinable!(versions -> crates (crate_id));
diesel::joinable!(versions -> users (published_by));
diesel::joinable!(versions_published_by -> versions (version_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_tokens,
    background_jobs,
    categories,
    crate_downloads,
    crate_infos,
    crate_owner_invitations,
    crate_owners,
    crate_results,
    crates,
    crates_categories,
    crates_keywords,
    dependencies,
    emails,
    follows,
    keywords,
    metadata,
    processed_log_files,
    publish_limit_buckets,
    publish_rate_overrides,
    readme_renderings,
    reserved_crate_names,
    teams,
    users,
    version_downloads,
    version_owner_actions,
    versions,
    versions_published_by,
);
