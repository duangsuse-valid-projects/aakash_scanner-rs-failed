#[macro_use]
extern crate lazy_static;
extern crate telebot;
extern crate rustbreak;
extern crate regex;
extern crate tokio_core;
extern crate futures;

use tokio_core::reactor::Core;
use futures::stream::Stream;
use futures::Future;
use std::fs::File;
use std::env::var;
use std::time::SystemTime;
use rustbreak::{Database, Result};
use telebot::bot;
use telebot::functions::*;

static API_VAR_NAME: &str = "TG_TOKEN";

lazy_static! {
    #[derive(Debug)]
    static ref APIKEY: String = {
        if let Ok(v) = var(&API_VAR_NAME) {v} else {
            println!("set bot token to {} first.", API_VAR_NAME);
            ::std::process::exit(1);
        }
    };
}

fn main() {
    println!(
        "aakash_scanner-rust is starting at unix time:{:?} with KEY:{}",
        SystemTime::now(),
        APIKEY.to_string()
    );
    // create a new event loop
    let mut lp = Core::new().unwrap();
    // init the bot with the bot key and an update interval of 200ms
    let bot = bot::RcBot::new(lp.handle(), &APIKEY).update_interval(200);

    // register a new command "reply" which replies all received messages
    let handle = bot.new_cmd("/reply").and_then(|(bot, msg)| {
        let mut text = msg.text.unwrap().clone();

        // when the text is empty send a dummy text
        if text.is_empty() {
            text = "Hello,telobot! Hello,world!".into();
        }

        // construct a message and return a new future which will be resolved by tokio
        bot.message(msg.chat.id, text).send()
    });

    // register the new command
    bot.register(handle);

    // start the event loop
    bot.run(&mut lp).unwrap();
}
