use tokio::io;
use tokio::sync::mpsc::Receiver;
use crate::error::ExeErr;
use crate::model::command::EngineRequest;
use crate::model::orderbook::OrderBook;
use crate::model::{ExeOk, Order, OrderbookCmd};

pub async fn run_orderbook_engine(mut orderbook: OrderBook, mut rx: Receiver<EngineRequest>) ->io::Result<()> {

    while let Some(req) = rx.recv().await {
        let resu = execute_cmd(req.cmd, &mut orderbook);
        if cfg!(debug_assertions) {
            orderbook
                .check_invariants()
                .expect("orderbook invariant violation");
        }
        let _ = req.reply.send(resu);
    }
    Ok(())
}

pub(crate) fn execute_cmd(cmd: OrderbookCmd, orderbook: &mut OrderBook) -> Result<ExeOk, ExeErr> {
    match cmd {
        OrderbookCmd::Add(o) => handle_add_order(o, orderbook),
        OrderbookCmd::Cancel(order_id) => cancel_order(orderbook,order_id),
        OrderbookCmd::Reduce { id, qty } => reduce_order(id, qty, orderbook),
        OrderbookCmd::Get(order_id) => get_order(orderbook,order_id),
        OrderbookCmd::Summary => summarize_orders(orderbook),
        // OrderbookCmd::Shutdown => Ok(ExeOk::Shutdown),
    }
}
pub(crate) fn handle_add_order(order: Order, orderbook: &mut OrderBook) -> Result<ExeOk, ExeErr> {
    orderbook.add_order(order)?;
    Ok(ExeOk::Added)
}
pub(crate) fn cancel_order(
    orderbook: &mut OrderBook,
    target_id: u32,
) -> Result<ExeOk, ExeErr> {
    orderbook.cancel_order(target_id)?;
    Ok(ExeOk::Canceled)
}

pub(crate) fn reduce_order(
    _id: u32,
    _qty: u32,
    _orderbook: &mut OrderBook,
) -> Result<ExeOk, ExeErr> {
    todo!("第25轮后续小轮实现新结构下的 reduce_order")
}

pub(crate) fn get_order(
    orderbook: &OrderBook,
    target_id: u32,
) -> Result<ExeOk, ExeErr> {
    let order = orderbook.get_order(target_id)?;
    Ok(ExeOk::Order(order))
}


pub(crate) fn summarize_orders(
    orderbook: &OrderBook,
) -> Result<ExeOk, ExeErr> {
    Ok(ExeOk::Summary( orderbook.summary()))
}
/*
pub(crate) fn add_order(order: Order, orderbook: &mut OrderBook) -> Result<ExeOk, ExeErr> {
    if orderbook.orders().iter().any(|o| o.id() == order.id()) {
        return Err(ExeErr::DuplicateOrderId {
            order_id: order.id(),
        });
    };
    orderbook.orders_mut().push(order);
    Ok(ExeOk::Added)
}

pub(crate) fn cancel_order(target_id: u32, orderbook: &mut OrderBook) -> Result<ExeOk, ExeErr> {
    if !orderbook.orders().iter().any(|o| o.id() == target_id) {
        return Err(ExeErr::OrderNotFound {
            order_id: target_id,
        });
    };
    orderbook.orders_mut().retain(|o| o.id() != target_id);

    Ok(ExeOk::Canceled)
}

pub(crate) fn reduce_order(id: u32, qty: u32, orderbook: &mut OrderBook) -> Result<ExeOk, ExeErr> {
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
        Ok(ExeOk::Reduced)
    } else {
        orderbook.orders_mut().remove(idx);
        Ok(ExeOk::Clear)
    }
}

pub(crate) fn get_order(target_id: u32, orderbook: &OrderBook) -> Result<ExeOk, ExeErr> {
    match orderbook.orders().iter().find(|o| o.id() == target_id) {
        Some(o) => Ok(ExeOk::Order(o.clone())),
        None => Err(ExeErr::OrderNotFound {
            order_id: target_id,
        }),
    }
}
pub(crate) fn summarize_orders(orderbook: &OrderBook) -> Result<ExeOk, ExeErr> {
    let mut buy_count = 0;
    let mut sell_count = 0;
    let mut total_value = 0;
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

    Ok(ExeOk::Summary(smr))
}
*/