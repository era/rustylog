use crate::config::{self, InputConfigOption};
use crate::plugin::Payload;
use crate::plugin::{input::PluginError, Context, InputPlugin};
use std::collections::HashMap;
use tokio::io::{self, AsyncBufReadExt, AsyncRead, BufReader, Lines, Stdin};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
struct IdGen {
    count: u64,
    name: String,
}

impl IdGen {
    fn new(name: String) -> Self {
        Self { count: 0, name }
    }

    fn next(&mut self) -> String {
        self.count += 1;
        format!("{}-{}", self.name, self.count)
    }
}

pub struct ReaderPlugin<R: AsyncRead + Unpin + Send + 'static> {
    config: HashMap<String, config::AttributeValue>,
    shutdown: Option<oneshot::Sender<()>>,
    reader: Option<Lines<BufReader<R>>>,
}

impl<R: AsyncRead + Unpin + Send + 'static> InputPlugin for ReaderPlugin<R> {
    /// start must be called with a reader in place, otherwise it will return
    /// `Err(PluginError::NotInitialized)`.
    fn start(
        &mut self,
        context: Context,
        channel: mpsc::UnboundedSender<Payload>,
    ) -> Result<(), PluginError> {
        let (cancel_tx, mut cancel_rx) = oneshot::channel::<()>();

        self.shutdown = Some(cancel_tx);

        let mut line_reader = if let Some(reader) = self.reader.take() {
            reader
        } else {
            return Err(PluginError::NotInitialized(
                "must initialize reader before calling start".to_owned(),
            ));
        };

        let mut id_gen = IdGen::new(self.identifier());
        let identifier = self.identifier();

        context.runtime.spawn(async move {
            loop {
                tokio::select! {
                    line = line_reader.next_line() => {
                        match line {
                            Ok(Some(data)) => {
                                channel.send(Payload {
                                    id: id_gen.next(),
                                    data,
                                    plugin_id: identifier.clone(),
                            }).expect("err while trying to send message");
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

        Ok(())
    }

    /// commit does not do anything as we do not store what was send before
    /// or is there a way to repeat the inputs.
    fn commit(&mut self, _: Context, _: String) -> Result<(), PluginError> {
        Ok(())
    }

    fn shutdown(&mut self, _: Context) -> Result<(), PluginError> {
        // if the channel is not there, it means we already send a message to
        // stop the operation. So nothing to do here.
        self.shutdown.take().and_then(|c| Some(c.send(())));
        Ok(())
    }

    fn identifier(&self) -> String {
        self.config
            .get(&InputConfigOption::Name.to_string())
            .map(|n| n.to_string())
            .unwrap_or("ReaderPlugin".to_string())
    }
}

/// StdinPlugin reads from the stdin and sends messages to the filters.
/// StdinPlugin does not try to resend messages in case something goes wrong,
/// and does not keep track of which messages were send or not. Clients writing to
/// stdin should retry the operation in case of failures. In other words, if the process
/// restarts, there could be data loss.
pub type StdinPlugin = ReaderPlugin<Stdin>;

impl StdinPlugin {
    pub fn new(config: HashMap<String, config::AttributeValue>) -> Self {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        let lines = reader.lines();
        let plugin = Self {
            config,
            shutdown: None,
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

        let (sender, mut receiver) = mpsc::unbounded_channel();

        let data = "This is a test\nWith multiple lines\n".as_bytes();

        let cursor = Cursor::new(data);
        let reader = BufReader::new(cursor);
        let lines = reader.lines();

        let mut plugin = ReaderPlugin {
            config: HashMap::new(),
            shutdown: None,
            reader: Some(lines),
        };

        plugin.start(ctx.clone(), sender).unwrap();

        assert_eq!("This is a test", receiver.recv().await.unwrap().data);
        assert_eq!("With multiple lines", receiver.recv().await.unwrap().data);

        plugin.shutdown(ctx.clone()).unwrap();
    }
}
