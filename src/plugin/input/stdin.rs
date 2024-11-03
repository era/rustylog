use crate::config::{self, AttributeValue, Plugin};
use crate::plugin::{input::PluginError, Context, InputPlugin};
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::{broadcast, oneshot};

/// StdinPlugin reads from the stdin and sends messages to the filters.
/// StdinPlugin does not try to resend messages in case something goes wrong,
/// and does not keep track of which messages were send or not. Clients writing to
/// stdin should retry the operation in case of failures. In order words, if the process
/// restarts, there could be data loss.
pub struct StdinPlugin {
    shutdown: Option<oneshot::Sender<()>>,
    sender: broadcast::Sender<String>,
}

impl InputPlugin for StdinPlugin {
    fn init(
        &mut self,
        context: Context,
        _config: Vec<(String, config::AttributeValue)>,
    ) -> Result<Self, PluginError> {
        let (cancel_tx, mut cancel_rx) = oneshot::channel::<()>();
        //TODO make the capacity configurable
        let (tx, mut rx1) = broadcast::channel(100);
        // we will generate consumers with the subscriber method
        drop(rx1);
        let plugin = StdinPlugin {
            shutdown: Some(cancel_tx),
            sender: tx.clone(),
        };

        context.runtime.spawn(async move {
            let stdin = io::stdin();
            let reader = BufReader::new(stdin);
            let mut lines = reader.lines();
            loop {
                tokio::select! {
                    line = lines.next_line() => {
                        match line {
                            Ok(Some(line)) => {
                                tx.send(line).context("err while trying to send message");
                            }
                            Ok(None) => {
                                // ignore if someone sends EOF
                                continue;
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

        Ok(plugin)
    }

    /// commit does not do anything as we do not store what was send before
    /// or is there a way to repeat the inputs.
    fn commit(&mut self, _: Context) -> Result<(), PluginError> {
        Ok(())
    }

    fn subscribe(&mut self, context: Context) -> Result<broadcast::Receiver<String>, PluginError> {
        Ok(self.sender.subscribe())
    }

    fn shutdown(&mut self, _: Context) -> Result<(), PluginError> {
        // if the channel is not there, it means we already send a message to
        // stop the operation. So nothing to do here.
        self.shutdown.take().and_then(|c| Some(c.send(())));
        Ok(())
    }
}
