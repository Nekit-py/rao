create schema if not exists  _menin;

CREATE TYPE _menin.order_type AS ENUM ('order', 'disposal');

CREATE TYPE _menin.status AS ENUM ('completed', 'in_progress');

CREATE TABLE IF NOT EXISTS _menin.ORDERS (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    number TEXT NOT NULL,
    deleted boolean NOT NULL DEFAULT FALSE,
    created timestamp NOT NULL default CURRENT_DATE,
    updated timestamp default null,
    order_type _menin.order_type NOT NULL default 'order',
    title varchar(80) NOT NULL,
    initiator varchar(70) NOT NULL,
    responsible_employee varchar(70) NOT NULL,
    deadline date NOT NULL,
    status _menin.status NOT NULL default 'in_progress',
    closed timestamp default null,
    comment TEXT default null
);
