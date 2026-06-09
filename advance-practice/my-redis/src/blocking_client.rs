use tokio::net::ToSocketAddrs;
use tokio::runtime::Runtime;

pub use crate::client::Message;

/// 建立到 redis 服务端的连接
pub struct BlockingClient {
    /// 之前实现的异步客户端 `Client`
    inner: crate::client::Client,

    /// 一个 `current_thread` 模式的 `tokio` 运行时，
    /// 使用阻塞的方式来执行异步客户端 `Client` 上的操作
    rt: Runtime,
}

pub fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<BlockingClient> {
    // 构建一个 tokio 运行时： Runtime
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;

    // 使用运行时来调用异步的连接方法
    let inner = rt.block_on(crate::client::connect(addr))?;

    Ok(BlockingClient { inner, rt })
}

use bytes::Bytes;
use std::time::Duration;

impl BlockingClient {
    pub fn get(&mut self, key: &str) -> crate::Result<Option<Bytes>> {
        self.rt.block_on(self.inner.get(key))
    }

    pub fn set(&mut self, key: &str, value: Bytes) -> crate::Result<()> {
        self.rt.block_on(self.inner.set(key, value))
    }

    pub fn set_expires(
        &mut self,
        key: &str,
        value: Bytes,
        expiration: Duration,
    ) -> crate::Result<()> {
        self.rt
            .block_on(self.inner.set_expires(key, value, expiration))
    }

    pub fn publish(&mut self, channel: &str, message: Bytes) -> crate::Result<u64> {
        self.rt.block_on(self.inner.publish(channel, message))
    }
}
