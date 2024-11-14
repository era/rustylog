use crate::config::{self, AttributeValue, Plugin};
use crate::plugin::{input::PluginError, Context, InputPlugin};
use std::collections::HashMap;
use tokio::io::{self, AsyncBufReadExt, AsyncRead, BufReader, Lines, Stdin};
use tokio::sync::{broadcast, oneshot};

#[derive(Default)]
pub struct ReaderPlugin<R: AsyncRead + Unpin + Send + 'static> {
    shutdown: Option<oneshot::Sender<()>>,
    sender: Option<broadcast::Sender<String>>,
    reader: Option<Lines<BufReader<R>>>,
}

impl<R: AsyncRead + Unpin + Send + 'static> InputPlugin for ReaderPlugin<R> {
    /// start must be called with a reader in place, otherwise it will return
    /// `Err(PluginError::NotInitialized)`.
    fn start(
        &mut self,
        context: Context,
        _: HashMap<String, config::AttributeValue>,
    ) -> Result<broadcast::Receiver<String>, PluginError> {
        let (cancel_tx, mut cancel_rx) = oneshot::channel::<()>();
        //TODO make the capacity configurable
        let (tx, rx1) = broadcast::channel(100);

        self.shutdown = Some(cancel_tx);
        self.sender = Some(tx.clone());

        let mut line_reader = if let Some(reader) = self.reader.take() {
            reader
        } else {
            return Err(PluginError::NotInitialized(
                "must initialize reader before calling start".to_owned(),
            ));
        };

        context.runtime.spawn(async move {
            loop {
                tokio::select! {
                    line = line_reader.next_line() => {
                        match line {
                            Ok(Some(line)) => {
                                tx.send(line).expect("err while trying to send message");
                            }
                            Ok(None) => {
                                break;
                            }
                            Err(e) => {
                                eprintln!("Error reading line: {}", e);
                                break;
                            }
                        }
                    }
                    _ = &mut cancel_rx => {
                        println!("Cancellation signal received. Stopping input read.");
                        break;
                    }
                }
            }
        });

        Ok(rx1)
    }

    /// commit does not do anything as we do not store what was send before
    /// or is there a way to repeat the inputs.
    fn commit(&mut self, _: Context) -> Result<(), PluginError> {
        Ok(())
    }

    fn subscribe(&mut self, _: Context) -> Result<broadcast::Receiver<String>, PluginError> {
        Ok(self
            .sender
            .as_ref()
            .expect("can only call subscribe in a initiated plugin")
            .subscribe())
    }

    fn shutdown(&mut self, _: Context) -> Result<(), PluginError> {
        // if the channel is not there, it means we already send a message to
        // stop the operation. So nothing to do here.
        self.shutdown.take().and_then(|c| Some(c.send(())));
        Ok(())
    }
}

/// StdinPlugin reads from the stdin and sends messages to the filters.
/// StdinPlugin does not try to resend messages in case something goes wrong,
/// and does not keep track of which messages were send or not. Clients writing to
/// stdin should retry the operation in case of failures. In other words, if the process
/// restarts, there could be data loss.
pub type StdinPlugin = ReaderPlugin<Stdin>;

impl StdinPlugin {
    pub fn default() -> Self {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let lines = reader.lines();
        let plugin = Self {
            shutdown: None,
            sender: None,
            reader: Some(lines),
        };
        plugin
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[tokio::test]
    async fn test_outputs_from_stdin() {
        let ctx = Context::new();

        let data = "This is a test\nWith multiple lines\n".as_bytes();

        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        let lines = reader.lines();

        let mut plugin = ReaderPlugin {
            shutdown: None,
            sender: None,
            reader: Some(lines),
        };
        let mut sub = plugin.start(ctx.clone(), HashMap::new()).unwrap();

        assert_eq!("This is a test", sub.recv().await.unwrap());
        assert_eq!("With multiple lines", sub.recv().await.unwrap());

        plugin.shutdown(ctx.clone()).unwrap();
    }
}
