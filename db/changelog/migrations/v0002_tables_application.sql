--liquibase formatted sql

--changeset johny:1
create table anothergtw.tb_application (
    id bigserial primary key,
    name varchar(100) not null,
    path varchar(255) not null,
    url_destination varchar(255) not null,
    created_dttm timestamptz not null,
    update_dttm timestamptz not null,
    constraint ta_path_uq unique (path)
);

--changeset johny:2
create table anothergtw.tb_application_rule (
    id bigserial primary key,
    id_application bigint not null,
    path varchar(255) not null,
    created_dttm timestamptz not null,
    update_dttm timestamptz not null,
    redirect_to varchar(255) not null,
    constraint ta_id_application_path_uq unique (id_application, path)
);