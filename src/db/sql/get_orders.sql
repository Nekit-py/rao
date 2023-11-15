SELECT
    id,
    number,
    deleted,
    created,
    updated,
    order_type as "order_type: OrderType",
    title,
    discription,
    initiator,
    responsible_employee,
    deadline,
    status as "status: OrderStatus",
    closed,
    comment
FROM _menin.orders
