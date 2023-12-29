#[derive(Debug)]
pub struct Backend;

impl<'backend> sophon_core::Backend<'backend> for Backend {
    type Error = error::Error;

    async fn query(
        &'backend mut self,
        message: Message,
        publisher: Arc<MessagePublisher<'backend>>,
    ) -> error::Result<()> {
        println!("{message}");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use tokio::{
        io::{self, AsyncBufReadExt, BufReader, BufWriter},
        time::Duration,
    };

    #[tokio::test]
    async fn tokio_stdin() {
        // let stdin = io::stdin();
        // let mut reader = BufReader::new(stdin);

        // for _ in 0..2 {
        //     let mut input = "".to_owned();
        //     let timeout = tokio::time::sleep(Duration::from_millis(5000));

        //     println!("say something:");

        //     tokio::select! {
        //         result = reader.read_line(&mut input) => {
        //             match result {
        //                 Ok(_bytes_read) => println!("{input}"),
        //                 Err(error) => eprintln!("Error: {error:?}"),
        //             }
        //         }
        //         _ = timeout => eprintln!("Took too long"),
        //     }
        // }
    }
}
