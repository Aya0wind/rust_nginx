use tokio::io::AsyncReadExt;
use futures::pin_mut;
use futures::future::{Select, Either};


pub async fn shell_handler() {
    let interrupt = tokio::signal::ctrl_c();
    let shell = async move {
        let mut buf = [0u8; 10];
        tokio::io::stdin().read(&mut buf).await?;
        Ok::<_, tokio::io::Error>(buf)
    };
    pin_mut!(interrupt);
    pin_mut!(shell);
    match futures::future::select(interrupt, shell).await {
        Either::Left(_)=>{
            eprintln!("Server interrupted by ctrl-c signal\nBye");
        }
        Either::Right(_)=>{
            eprintln!("Server exit!");
        }
    }
}
