
create table external_auth_source
(
        external_auth_source_id serial primary key,
        source_name varchar(255) not null,
        constraint ux_external_auth_source_name unique(source_name)
);

create table traveler
(
        traveler_id serial primary key,
        external_auth_source_id int not null foreign key references external_auth(external_auth_source_id),
        external_auth_id varchar(255) not null,
        constraint ux_traveler_external_auth unique(external_auth_source_id, external_auth_id)
);
