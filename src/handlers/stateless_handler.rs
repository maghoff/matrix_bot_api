use std::collections::HashMap;
use std::cmp::min;
use handlers::{Message, MessageHandler, extract_command, HandleResult};
use MatrixBot;

/// Convenience-handler that can quickly register and call functions
pub struct StatelessHandler {
    cmd_prefix: String,
    cmd_handles: HashMap<String, Box<dyn Fn(&MatrixBot, &Message, &str) -> HandleResult>>,
}

impl StatelessHandler {
    pub fn new() -> StatelessHandler {
        StatelessHandler{cmd_prefix: "!".to_string(),
                      cmd_handles: HashMap::new()}
    }

    /// With what prefix commands to the bot will start
    /// Default: "!"
    pub fn set_cmd_prefix(&mut self, prefix: &str) {
        self.cmd_prefix = prefix.to_string();
    }

    /// Register handles
    /// * command: For which command (excluding the prefix!) the handler should be called
    /// * handler: The handler to be called if the given command was received in the room
    ///
    /// Handler-function:
    /// * bot:     This bot
    /// * room:    The room the command was sent in
    /// * command: The message-body without prefix and command (e.g. "!roll 12" -> "12")
    ///
    /// # Example
    /// handler.set_cmd_prefix("BOT:")
    /// handler.register_handle("sayhi", foo);
    /// foo() will be called, when BOT:sayhi is received by the bot
    pub fn register_handle(&mut self,
                           command: &str,
                           handler: Box<dyn Fn(&MatrixBot, &Message, &str) -> HandleResult>)
    {
        self.cmd_handles.insert(command.to_string(), handler);
    }
}

impl MessageHandler for StatelessHandler {
    fn handle_message(&mut self, bot: &MatrixBot, message: &Message) -> HandleResult {
        match extract_command(&message.body, &self.cmd_prefix) {
            Some(command) => {
                                let func = self.cmd_handles.get(command);
                                match func {
                                    Some(func) => {
                                        if bot.verbose {
                                            println!("Found handle for command \"{}\". Calling it.", &command);
                                        }
                                        let end_of_prefix = self.cmd_prefix.len() + command.len();
                                        func(bot, message, &message.body[min(end_of_prefix+1, message.body.len())..])
                                    }
                                    None => {
                                        if bot.verbose {
                                            println!("Command \"{}\" not found in registered handles", &command);
                                        }
                                        HandleResult::ContinueHandling
                                    }
                                }
                            }
            None => { HandleResult::ContinueHandling /* Doing nothing. Not for us */}
        }
   }
}
