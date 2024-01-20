-- Add migration script here
set application_name = 'user-rs';

drop table if exists users;

create table if not exists users
(
    id         serial8 primary key         not null,
    username   varchar(64) unique          not null default '',
    status     bool                        not null default false,
    root       bool                        not null default false,
    email      varchar(64)                          default '',
    phone      varchar(64)                          default '',
    created_by bigint                      not null default 1,
    updated_by bigint                      not null default 1,
    deleted_by bigint                      not null default 0,
    created_at timestamp without time zone not null default now(),
    updated_at timestamp without time zone not null default now(),
    deleted_at timestamp without time zone
);

INSERT INTO public.users (id, username, status, root, email, phone, created_by, updated_by, deleted_by, created_at,
                          updated_at, deleted_at)
VALUES (1, 'admin', true, true, '', '', 1, 1, 0, '2024-01-18 09:11:26.906833', '2024-01-18 09:11:26.906833', null);

