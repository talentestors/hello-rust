use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("foo.txt").await?;
    let mut buffer = Vec::new();

    // 读取整个文件的内容
    f.read_to_end(&mut buffer).await?;
    println!("{}", unsafe { String::from_utf8_unchecked(buffer) });
    Ok(())
}
