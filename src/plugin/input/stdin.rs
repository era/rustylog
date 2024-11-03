use crate::config::{self, AttributeValue, Plugin};
use crate::plugin::{input::PluginError, Context, InputPlugin};
use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::sync::oneshot;
use tokio::sync::oneshot::Sender;

/// StdinPlugin reads from the stdin and sends messages to the filters.
/// StdinPlugin does not try to resend messages in case something goes wrong,
/// and does not keep track of which messages were send or not. Clients writing to
/// stdin should retry the operation in case of failures. In order words, if the process
/// restarts, there could be data loss.
pub struct StdinPlugin {
    shutdown: Option<Sender<()>>,
}

impl InputPlugin for StdinPlugin {
    fn init(
        &mut self,
        context: Context,
        _config: Vec<(String, config::AttributeValue)>,
    ) -> Result<Self, PluginError> {
        let (cancel_tx, mut cancel_rx) = oneshot::channel::<()>();

        let plugin = StdinPlugin {
            shutdown: Some(cancel_tx),
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
                                println!("Read line: {}", line);
                            }
                            Ok(None) => {
                                println!("End of input");
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

        Ok(plugin)
    }

    /// commit does not do anything as we do not store what was send before
    /// or is there a way to repeat the inputs.
    fn commit(&mut self, _: Context) -> Result<(), PluginError> {
        Ok(())
    }

    fn producer(&mut self, context: Context) -> Result<(), PluginError> {
        todo!()
    }

    fn shutdown(&mut self, _: Context) -> Result<(), PluginError> {
        // if the channel is not there, it means we already send a message to
        // stop the operation. So nothing to do here.
        self.shutdown.take().and_then(|c| Some(c.send(())));
        Ok(())
    }
}
