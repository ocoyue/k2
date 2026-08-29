use crate::handler::OrderHandler;
use protocol::{OrderCodec, OrderResponse};
use std::io;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::{self, Instant};
/*
  Why use reader.lines() ?
  ->  cancellation safety:
      Future 即使在 .await 尚未完成时被 drop，也不会导致下一次重新执行该操作时出现数据丢失或状态损坏。
      loser_branch 状态不会消失，到下一轮的时候，状态依然可以被唤醒
      tokio::select!{
          branch1 ...
          branch2 ...
              }
      一个 Task
          │
          ├── poll Future A
          └── poll Future B
      同一个 Task 同时等待多个事件源，谁先 Ready 就处理谁.

  Why need to considering cancellation safety when session reading?
  ->  以后 TCP read 会和 heartbeat / shutdown 等 Future 共同进入 select!。
  TCP read 可能成为 loser 并被反复取消。
*/
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);

// TS-014:
// select! heartbeat branch 已建立，
// TS-015 再正式启用并实现 heartbeat 语义。
const HEARTBEAT_BRANCH_ENABLED: bool = false;

pub struct OrderSession<H> {
    stream: TcpStream,
    handler: H,
}

impl<H> OrderSession<H>
where
    H: OrderHandler,
{
    pub fn new(stream: TcpStream, handler: H) -> Self {
        Self { stream, handler }
    }

    pub async fn run(self) -> io::Result<()> {
        let OrderSession {
            mut stream,
            handler,
        } = self;

        let (read_half, mut write_half) = stream.split();

        let reader = BufReader::new(read_half);
        let mut lines = reader.lines();

        let first_heartbeat_at =
            Instant::now() + HEARTBEAT_INTERVAL;

        let mut heartbeat_tick =
            time::interval_at(
                first_heartbeat_at,
                HEARTBEAT_INTERVAL,
            );

        loop {
            tokio::select! {
            frame_result = lines.next_line() => {
                let frame_result = frame_result?;

                let Some(frame) = frame_result else {
                    println!("order client disconnected");
                    break;
                };

                let request_result =
                    OrderCodec::decode(frame.trim());

                let response = match request_result {
                    Ok(request) => {
                        handler.handle(request).await
                    }

                    Err(message) => {
                        OrderResponse::Error { message }
                    }
                };

                let output =
                    OrderCodec::encode(response);

                write_half
                    .write_all(output.as_bytes())
                    .await?;
            }

            _ = heartbeat_tick.tick() => {
                todo!("TS-015: implement heartbeat protocol");
            }
        }
        }

        Ok(())
    }
}
