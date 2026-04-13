use crate::error::{BusinessError, ExecuteErr};
use crate::model::{Command, Order};

pub fn execute_cmd(cmd:Command,orders:&mut Vec<Order>)->Result<(),ExecuteErr>{
    match cmd {
        Command::ADD(o) => add_order(o,orders),
    }
    
}

pub(crate) fn add_order(order:Order,orders:&mut Vec<Order>)->Result<(),ExecuteErr>{
    if orders.iter().any(|o| o.id() == order.id()){
        return Err(ExecuteErr::DuplicateOrderId {order_id:order.id()})?;
    };
    let old_len = orders.len();
    orders.push(order);
    if orders.len() > old_len{
        Ok(())
    }else { 
        Err(ExecuteErr::Internal("Vec<Order> pushing failed.".to_string()))
    }
}









