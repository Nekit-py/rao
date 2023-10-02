
create schema if not exists  _menin;

CREATE TYPE _mening.order_type AS ENUM ('Приказ', 'Распоряжение');

CREATE TYPE _mening.status AS ENUM ('Закрыт', 'На исполнении');


CREATE TABLE IF NOT EXISTS _mening.ORDERS (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    number TEXT NOT NULL,
    deleted boolean NOT NULL DEFAULT FALSE,
    create_date timestamp NOT NULL default CURRENT_DATE,
    update_date timestamp default null,
    type _mening.order_type NOT NULL,
    title varchar(80) NOT NULL,
    initiator varchar(70) NOT NULL,
    responsible_employee varchar(70) NOT NULL,
    deadline date NOT NULL,
    status _mening.status NOT NULL,
    close_date timestamp default null,
    comment TEXT default null,
);