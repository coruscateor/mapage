//use

use std::future::Future;

use tokio::task::JoinHandle;


pub async fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
{

    tokio::spawn(future)

}
