use crate::error::ExeErr;
use crate::model::{Command, ExeResult, Order, Side, Summary};

pub fn execute_cmd(cmd: Command, orders: &mut Vec<Order>) -> Result<ExeResult, ExeErr> {
    match cmd {
        Command::Add(o) => add_order(o, orders),
        Command::Cancel(order_id) => cancel_order(order_id, orders),
        Command::Reduce { id, qty } => reduce_order(id, qty, orders),
        Command::Get(order_id) => get_order(order_id, orders),
        Command::Summary => count_order(orders),
    }
}
pub(crate) fn add_order(order: Order, orders: &mut Vec<Order>) -> Result<ExeResult, ExeErr> {
    if orders.iter().any(|o| o.id() == order.id()) {
        return Err(ExeErr::DuplicateOrderId {
            order_id: order.id(),
        });
    };
    orders.push(order);
    Ok(ExeResult::Added)
}

pub(crate) fn cancel_order(target_id: u32, orders: &mut Vec<Order>) -> Result<ExeResult, ExeErr> {
    if !orders.iter().any(|o| o.id() == target_id) {
        return Err(ExeErr::OrderNotFound {
            order_id: target_id,
        });
    };
    orders.retain(|o| o.id() != target_id);

    Ok(ExeResult::Canceled)
}

pub(crate) fn reduce_order(
    id: u32,
    qty: u32,
    orders: &mut Vec<Order>,
) -> Result<ExeResult, ExeErr> {
    let order_idx = orders.iter().position(|o| o.id() == id);
    let idx = match order_idx {
        Some(idx) => idx,
        None => return Err(ExeErr::OrderNotFound { order_id: id }),
    };

    if qty > orders[idx].qty() {
        Err(ExeErr::QuantityNotEnough {
            request: qty,
            available: orders[idx].qty(),
        })
    } else if qty < orders[idx].qty() {
        let new_qty = orders[idx].qty() - qty;
        orders[idx].set_qty(new_qty);
        Ok(ExeResult::Reduced)
    } else {
        orders.remove(idx);
        Ok(ExeResult::Clear)
    }
}

pub(crate) fn get_order(target_id: u32, orders: &[Order]) -> Result<ExeResult, ExeErr> {
    match orders.iter().find(|o| o.id() == target_id) {
        Some(o) => Ok(ExeResult::Order(o.clone())),
        None => Err(ExeErr::OrderNotFound {
            order_id: target_id,
        }),
    }
}
pub(crate) fn count_order(orders: &[Order]) -> Result<ExeResult, ExeErr> {
    let mut buy_count = 0;
    let mut sell_count = 0;
    let mut total_value = 0.0;
    for o in orders.iter() {
        match o.side() {
            Side::Buy => buy_count += 1,
            Side::Sell => sell_count += 1,
        }
        total_value += o.value();
    }

    let smr = Summary {
        orders_count: orders.len() as u32,
        buy_count,
        sell_count,
        total_value,
    };

    Ok(ExeResult::Summary(smr))
}
