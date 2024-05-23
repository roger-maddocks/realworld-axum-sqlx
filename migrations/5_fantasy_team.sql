create table fantasy_team
(
    team_id uuid primary key default uuid_generate_v1mc(),
    user_id uuid,
    username      text collate "case_insensitive" unique not null,
    email         text collate "case_insensitive" unique not null,
    bio           text                                   not null default '',
    image         text,
    created_at    timestamptz                            not null default now(),
    updated_at    timestamptz
);

-- And applying our `updated_at` trigger is as easy as this.
SELECT trigger_updated_at('fantasy_team');
