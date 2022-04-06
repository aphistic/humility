// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! ## `humility repl`
//!
//! read, eval, print, loop

use std::borrow::Cow;

use anyhow::Result;
use clap::Command as ClapCommand;
use humility_cmd::{
    ArchiveRequired, Attach, AttachementMetadata, Command, Validate,
};
use reedline::{PromptHistorySearchStatus, Reedline, Signal};

use crate::cmd;

struct Prompt;

impl reedline::Prompt for Prompt {
    fn render_prompt_left(&self) -> Cow<str> {
        Cow::from("humility")
    }

    fn render_prompt_right(&self) -> Cow<str> {
        Cow::default()
    }

    fn render_prompt_indicator(
        &self,
        _prompt_mode: reedline::PromptEditMode,
    ) -> Cow<str> {
        // not getting fancy for now
        Cow::from("> ")
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        Cow::from("... ")
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: reedline::PromptHistorySearch,
    ) -> Cow<str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };

        Cow::Owned(format!(
            "({}reverse-search: {}) ",
            prefix, history_search.term
        ))
    }
}

fn repl(context: &mut humility::ExecutionContext) -> Result<()> {
    let mut line_editor = Reedline::create()?;
    let prompt = Prompt;

    println!("Welcome to the humility REPL! Try out some subcommands, or 'quit' to quit!");

    loop {
        let sig = line_editor.read_line(&prompt)?;

        match sig {
            Signal::Success(buffer) => {
                let result = eval(context, &buffer)?;
                println!("{result}");
            }
            Signal::CtrlD | Signal::CtrlC => {
                println!("\nAborted!");
                break Ok(());
            }
            Signal::CtrlL => {
                line_editor.clear_screen().unwrap();
            }
        }
    }
}

fn eval(
    context: &mut humility::ExecutionContext,
    input: &str,
) -> Result<String> {
    match input.trim() {
        "quit" => {
            println!("Quitting!");
            std::process::exit(0);
        }
        user_input => {
            let mut input = vec!["humility"];
            input.extend(user_input.split(' '));

            let (commands, _, cli) = crate::parse_args(input);
            context.cli = cli;
            if let Err(e) = cmd::subcommand(context, &commands) {
                Ok(format!(
                    "I'm sorry, Dave. I'm afraid I can't understand that: '{e}'",
                ))
            } else {
                Ok(String::new())
            }
        }
    }
}

pub fn init() -> (Command, ClapCommand<'static>) {
    (
        Command {
            name: "repl",
            archive: ArchiveRequired::Required,
            attatchment_metadata: Some(AttachementMetadata {
                attach: Attach::Any,
                validate: Validate::Match,
            }),
            run: repl,
        },
        ClapCommand::new("repl"),
    )
}
