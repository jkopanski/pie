extern crate pie;
extern crate rustyline;

use std::{
    format,
    println,
    borrow::Cow::{
	Borrowed,
	Owned,
    },
};
use miette::Report;
use rustyline::{
    Cmd,
    CompletionType,
    Context,
    Editor,
    EditMode,
    KeyEvent,
    completion::{Completer, FilenameCompleter, Pair},
    config::Config,
    error::ReadlineError,
    highlight::{Highlighter, MatchingBracketHighlighter},
    validate::{Validator, MatchingBracketValidator},
    hint::{Hinter, HistoryHinter},
};
use rustyline_derive::Helper;
use xdg::{self, BaseDirectories};

use pie::error as error;
use pie::parser as parser;

#[derive(Helper)]
struct PieHelper {
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    validator: MatchingBracketValidator,
    hinter: HistoryHinter,
    colored_prompt: String,
}

impl Completer for PieHelper {
    type Candidate = Pair;

    fn complete(
	&self,
	line: &str,
	pos: usize,
	ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
	self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for PieHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
	self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for PieHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> std::borrow::Cow<'b, str> {
	if default {
	    Borrowed(&self.colored_prompt)
	} else {
	    Borrowed(prompt)
	}
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
	Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> std::borrow::Cow<'l, str> {
	self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize, forced: bool) -> bool {
	self.highlighter.highlight_char(line, pos, forced)
    }
}

impl Validator for PieHelper {
    fn validate(
	&self,
	ctx: &mut rustyline::validate::ValidationContext
    ) -> rustyline::Result<rustyline::validate::ValidationResult> {
	self.validator.validate(ctx)
    }

    fn validate_while_typing(&self) -> bool {
	self.validator.validate_while_typing()
    }
}

fn main() -> error::Result<()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("pie")?;
    let history_file = xdg_dirs.place_state_file("history")?;

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .auto_add_history(true)
        .build();
    let helper = PieHelper {
	completer: FilenameCompleter::new(),
	highlighter: MatchingBracketHighlighter::new(),
	hinter: HistoryHinter::new(),
	colored_prompt: "".to_owned(),
	validator: MatchingBracketValidator::new(),
    };
    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(helper));
    rl.bind_sequence(KeyEvent::alt('N'), Cmd::HistorySearchForward);
    rl.bind_sequence(KeyEvent::alt('P'), Cmd::HistorySearchBackward);
    let res  = rl.load_history(&history_file);

    if let Result::Err(err) = res {
	println!("Couldn't load previous history: {}.", err);
    }

    println!();
    println!("Pie lang repl");
    println!("Press Ctrl-D or enter \"quit\" to exit.");
    println!();

    let prompt = format!("ΛΠ ≫ ");
    rl.helper_mut()
	.expect("No helper")
	.colored_prompt = format!("\x1b[1;32m{}\x1b[0m", prompt);

    for readline in rl.iter (&prompt) {
        match readline {
	    Ok(line) => {
		if "quit" == line {
		    break;
		}

		match parser::parse(line.clone()) {
		    Ok(module) => {
			for stmt in module.statements.iter() {
			    println!("{stmt:#?}");
			}
		    },

		    Err(err) => {
			let report = Report::new(err).with_source_code(line);
			println!("{:?}", report);
		    }
		}
	    },

	    Err(err) => {
		match err {
		    ReadlineError::Interrupted =>
			println!("CTRL-C"),

		    ReadlineError::Eof =>
			println!("CTRL-D"),

		    e =>
			println!("{:?}", e)
		}
	        break;
	    }
	}
    }

  rl.append_history(&history_file)?;
  Ok(())
}
