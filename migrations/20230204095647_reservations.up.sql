-- Add up migration script here
CREATE TYPE rsvp.reservation_status AS ENUM('unknown','pending', 'confirmed', 'blocked');
CREATE TYPE rsvp.reservation_update_type AS ENUM('unknown','create', 'update', 'delete');

CREATE TABLE rsvp.reservations (
    id uuid NOT NULL default gen_random_uuid(),
    user_id varchar(64) NOT NULL ,
    status rsvp.reservation_status NOT NULL default 'pending',
    resource_id varchar(64) NOT NULL,
    timespan tstzrange NOT NULL,
    note TEXT,

    CONSTRAINT reservations_pkey PRIMARY KEY (id),
    CONSTRAINT reservations_conflict EXCLUDE USING gist (resource_id WITH =, timespan WITH &&)
);

CREATE INDEX reservations_resource_id_idx ON rsvp.reservations (resource_id);
CREATE INDEX reservations_user_id_idx ON rsvp.reservations (user_id);