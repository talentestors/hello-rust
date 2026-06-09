use tokio::signal;

#[tokio::main]
async fn main() {
    // ... spawn application as separate task ...
    // 在一个单独的任务中处理应用逻辑

    match signal::ctrl_c().await {
        Ok(()) => {
            println!("shutdown");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }

    //  发送关闭信号给应用所在的任务，然后等待
}
