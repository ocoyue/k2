use crate::error::ExeErr;
use crate::model::orderbook::OrderBook;
use crate::model::{Command, ExeResult, Order, Side, Summary};

pub fn execute_cmd(cmd: Command, orderbook: &mut OrderBook) -> Result<ExeResult, ExeErr> {
    match cmd {
        Command::Add(o) => add_order(o, orderbook),
        Command::Cancel(order_id) => cancel_order(order_id, orderbook),
        Command::Reduce { id, qty } => reduce_order(id, qty, orderbook),
        Command::Get(order_id) => get_order(order_id, orderbook),
        Command::Summary => count_order(orderbook),
    }
}
pub(crate) fn add_order(order: Order, orderbook: &mut OrderBook) -> Result<ExeResult, ExeErr> {
    if orderbook.orders().iter().any(|o| o.id() == order.id()) {
        return Err(ExeErr::DuplicateOrderId {
            order_id: order.id(),
        });
    };
    orderbook.orders_mut().push(order);
    Ok(ExeResult::Added)
}

pub(crate) fn cancel_order(target_id: u32, orderbook: &mut OrderBook) -> Result<ExeResult, ExeErr> {
    if !orderbook.orders().iter().any(|o| o.id() == target_id) {
        return Err(ExeErr::OrderNotFound {
            order_id: target_id,
        });
    };
    orderbook.orders_mut().retain(|o| o.id() != target_id);

    Ok(ExeResult::Canceled)
}

pub(crate) fn reduce_order(
    id: u32,
    qty: u32,
    orderbook: &mut OrderBook,
) -> Result<ExeResult, ExeErr> {
    let order_idx = orderbook.orders().iter().position(|o| o.id() == id);
    let idx = match order_idx {
        Some(idx) => idx,
        None => return Err(ExeErr::OrderNotFound { order_id: id }),
    };

    if qty > orderbook.orders()[idx].qty() {
        Err(ExeErr::QuantityNotEnough {
            request: qty,
            available: orderbook.orders()[idx].qty(),
        })
    } else if qty < orderbook.orders()[idx].qty() {
        let new_qty = orderbook.orders()[idx].qty() - qty;
        orderbook.orders_mut()[idx].set_qty(new_qty);
        Ok(ExeResult::Reduced)
    } else {
        orderbook.orders_mut().remove(idx);
        Ok(ExeResult::Clear)
    }
}

pub(crate) fn get_order(target_id: u32, orderbook: &OrderBook) -> Result<ExeResult, ExeErr> {
    match orderbook.orders().iter().find(|o| o.id() == target_id) {
        Some(o) => Ok(ExeResult::Order(o.clone())),
        None => Err(ExeErr::OrderNotFound {
            order_id: target_id,
        }),
    }
}
pub(crate) fn count_order(orderbook: &OrderBook) -> Result<ExeResult, ExeErr> {
    let mut buy_count = 0;
    let mut sell_count = 0;
    let mut total_value = 0.0;
    for o in orderbook.orders().iter() {
        match o.side() {
            Side::Buy => buy_count += 1,
            Side::Sell => sell_count += 1,
        }
        total_value += o.value();
    }

    let smr = Summary {
        orders_count: orderbook.orders().len() as u32,
        buy_count,
        sell_count,
        total_value,
    };

    Ok(ExeResult::Summary(smr))
}
