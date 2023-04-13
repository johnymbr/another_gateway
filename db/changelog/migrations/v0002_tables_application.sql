--liquibase formatted sql

--changeset johny:1
create table anothergtw.tb_application (
    id bigserial primary key,
    name varchar(100) not null,
    created_at timestamptz not null,
    updated_at timestamptz not null
);

--changeset johny:2
create table anothergtw.tb_application_workflow (
    id bigserial primary key,
    id_application bigint not null,
    path varchar(255) not null,
    forward_to varchar(255) not null,
    status varchar(50) not null,
    created_at timestamptz not null,
    updated_at timestamptz not null,
    constraint uq_taw_id_application_path unique (id_application, path),
    constraint fk_taw_id_application foreign key(id_application) references anothergtw.tb_application(id)
);

--changeset johny:3
create table anothergtw.tb_application_route (
    id bigserial primary key,
    id_application_workflow bigint,
    path varchar(255) not null,
    forward_to varchar(255),
    created_at timestamptz not null,
    updated_at timestamptz not null,
    constraint fk_tar_id_application_workflow foreign key(id_application_workflow) references anothergtw.tb_application_workflow(id)
);

--changeset johny:4
create table anothergtw.tb_application_orchestration (
    id bigserial primary key,
    id_application_workflow bigint not null,
    path varchar(255) not null,
    type varchar(1) not null constraint type_check check(type in ('P', 'S')),
    created_at timestamptz not null,
    updated_at timestamptz not null,
    constraint uq_tao_id_application_workflow_path unique (id_application_workflow, path),
    constraint fk_tao_id_application_workflow foreign key(id_application_workflow) references anothergtw.tb_application_workflow(id)
);

--changeset johny:5
create table anothergtw.tb_application_orchestration_route (
    id bigserial primary key,
    id_application_orchestration bigint not null,
    id_application_route bigint not null,
    response_key varchar(255) not null,
    constraint fk_taor_id_application_orchestration foreign key(id_application_orchestration) references anothergtw.tb_application_orchestration(id),
    constraint fk_taor_id_application_route foreign key(id_application_route) references anothergtw.tb_application_route(id)
);