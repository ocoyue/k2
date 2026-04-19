use crate::error::{BusinessError, ExecuteErr};
use crate::model::{Command, ExecuteResult, Order, Side, Summary};

pub fn execute_cmd(cmd: Command, orders: &mut Vec<Order>) -> Result<ExecuteResult, ExecuteErr> {
    match cmd {
        Command::ADD(o) => add_order(o, orders),
        Command::CANCEL(order_id) => cancel_order(order_id, orders),
        Command::REDUCE { id, qty } => reduce_order(id, qty, orders),
        Command::GET(order_id) => get_order(order_id, orders),
        Command::SUMMARY => count_order(orders),
    }
}

pub(crate) fn add_order(
    order: Order,
    orders: &mut Vec<Order>,
) -> Result<ExecuteResult, ExecuteErr> {
    if orders.iter().any(|o| o.id() == order.id()) {
        return Err(ExecuteErr::DuplicateOrderId {
            order_id: order.id(),
        })?;
    };
    let old_len = orders.len();
    orders.push(order);
    if orders.len() > old_len {
        Ok(ExecuteResult::Added)
    } else {
        Err(ExecuteErr::Internal(
            "Vec<Order> pushing failed.".to_string(),
        ))
    }
}

pub(crate) fn cancel_order(
    target_id: u32,
    orders: &mut Vec<Order>,
) -> Result<ExecuteResult, ExecuteErr> {
    if !orders.iter().any(|o| o.id() == target_id) {
        return Err(ExecuteErr::OrderNotFound {
            order_id: target_id,
        });
    };
    let old_len = orders.len();
    orders.retain(|o| o.id() != target_id);
    if old_len > orders.len() {
        Ok(ExecuteResult::Canceled)
    } else {
        Err(ExecuteErr::Internal(
            "Vec<Order> remaining failed.".to_string(),
        ))
    }
}

pub(crate) fn reduce_order(
    id: u32,
    qty: u32,
    orders: &mut Vec<Order>,
) -> Result<ExecuteResult, ExecuteErr> {
    if !orders.iter().any(|o| o.id() == id) {
        return Err(ExecuteErr::OrderNotFound { order_id: id });
    };
    let order_idx = orders.iter().position(|o| o.id() == id).unwrap();
    if qty > orders[order_idx].qty() {
        Err(ExecuteErr::QuantityNotEnough(qty))
    } else if qty < orders[order_idx].qty() {
        let new_qty = orders[order_idx].qty() - qty;
        orders[order_idx].set_qty(new_qty);
        Ok(ExecuteResult::Reduced)
    } else {
        orders.remove(order_idx);
        Ok(ExecuteResult::Deleted)
    }
}

pub(crate) fn get_order(target_id: u32, orders: &[Order]) -> Result<ExecuteResult, ExecuteErr> {
    match orders.iter().find(|o| o.id() == target_id) {
        Some(o) => Ok(ExecuteResult::Order(o.clone())),
        None => Err(ExecuteErr::OrderNotFound {
            order_id: target_id,
        }),
    }
}
pub(crate) fn count_order(orders: &[Order]) -> Result<ExecuteResult, ExecuteErr> {
    let mut buy_count = 0;
    let mut sell_count = 0;
    let mut total_value = 0.0;
    for o in orders.iter() {
        match o.side() {
            Side::BUY => buy_count += 1,
            Side::SELL => sell_count += 1,
        }
        total_value += o.value();
    }

    let smr = Summary {
        count: orders.len() as u32,
        buy_count,
        sell_count,
        total_value,
    };

    Ok(ExecuteResult::Summary(smr))
}
