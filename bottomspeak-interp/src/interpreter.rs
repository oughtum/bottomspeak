use crossterm::{
    QueueableCommand,
    cursor::{Hide, MoveTo},
    terminal::{Clear, ClearType},
};
use owo_colors::OwoColorize;

use crate::{env::EnvVars, lexer::Lexer, source::SourceContext};
use std::{
    collections::BTreeMap,
    error::Error,
    fmt,
    io::{Write, stdin, stdout},
    sync::LazyLock,
};

pub fn run(source: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx = SourceContext::new(source, name)?;

    let mut lexer = Lexer::new(&mut ctx);
    lexer.lex_tokens();
    // println!("{:?}", lexer.tokens());

    if ctx.err_occurred() {
        let mut name = ctx.rand_interp_title().to_string();
        let name = format!("{}{}", name.remove(0).to_uppercase(), name);

        ctx.output_errors()?;
        println!(
            "{}{}{}{}{}{}",
            name.magenta(),
            " found some errors in your code but it's okay, ".magenta(),
            ctx.rand_petname().magenta(),
            ", ".magenta(),
            ctx.rand_interp_title().magenta(),
            " believes in you <3\n".magenta(),
        );
        return Ok(());
    }

    println!(
        "{}{}{}{}{}",
        "Your code had no errors, ".magenta(),
        ctx.rand_interp_title().magenta(),
        " is so proud of you, ".magenta(),
        ctx.rand_petname().magenta(),
        " <3".magenta(),
    );

    Ok(())
}

static REPL_CMDS: LazyLock<BTreeMap<char, ReplCmd>> = LazyLock::new(|| {
    BTreeMap::from([
        ('q', ReplCmd::Quit),
        ('h', ReplCmd::Help),
        ('r', ReplCmd::Run),
        ('c', ReplCmd::Clear),
    ])
});
pub fn repl() -> Result<(), Box<dyn Error>> {
    let env_vars = EnvVars::new();

    println!(
        "{}{}{}{}{}",
        "Hi ".yellow(),
        env_vars.rand_petname().yellow(),
        "~\nType .q to quit, or .h for a list of things ".yellow(),
        env_vars.rand_interp_title().yellow(),
        " can do for you~".yellow(),
    );

    let mut state = ReplState::default();
    let mut stdout = stdout();

    clear_screen(false);

    loop {
        if state.quit {
            break;
        }

        let mut line = String::new();
        print!("{}", "~ ".blue());
        stdout.flush().unwrap();
        stdin().read_line(&mut line).unwrap();

        if line == "\n" {
            state.src.push('\n');
            continue;
        }

        if !line.starts_with('.') {
            state.src.push_str(&line);
            stdout.flush().unwrap();
            continue;
        }

        let cmdkey = line.trim_start_matches(".").trim_end().chars().next();

        if cmdkey.is_none() {
            println!(
                "{}{}{}{}{}",
                "Be a good ".yellow(),
                env_vars.rand_praise_honorific().yellow(),
                " and use your words to tell ".yellow(),
                env_vars.rand_interp_title().yellow(),
                " what you want me to do~".yellow()
            );
            continue;
        }

        let cmdkey = cmdkey.unwrap();

        if let Some(cmd) = REPL_CMDS.get(&cmdkey) {
            handle_repl_cmd(*cmd, &mut state)?;
        } else {
            println!(
                "{}{}{}{}{}{}",
                env_vars.rand_interp_title().yellow(),
                " doesn't understand `.".yellow(),
                cmdkey.yellow(),
                "`, ".yellow(),
                env_vars.rand_petname().yellow(),
                "~".yellow()
            );
        }
    }

    stdout.flush()?;

    if state.src.is_empty() {
        return Ok(());
    }

    Ok(())
}

fn handle_repl_cmd(cmd: ReplCmd, state: &mut ReplState) -> Result<(), Box<dyn Error>> {
    match cmd {
        ReplCmd::Quit => {
            state.src = String::new();
            state.quit = true;
        }
        ReplCmd::Help => {
            println!("{}", "Commands:".yellow());
            for (cmdkey, cmd) in REPL_CMDS.iter() {
                println!("{}{}\t{}", '.'.blue(), cmdkey.blue(), cmd.green())
            }
        }
        ReplCmd::Run => {
            run(&state.src, "<repl>")?;
            state.src = String::new();
        }
        ReplCmd::Clear => {
            state.src = String::new();
            clear_screen(true);
        }
    }

    Ok(())
}

fn clear_screen(full: bool) {
    let mut out = stdout();
    out.queue(Hide).unwrap();

    if full {
        out.queue(Clear(ClearType::All)).unwrap();
        out.queue(MoveTo(0, 0)).unwrap();
    } else {
        out.queue(Clear(ClearType::Purge)).unwrap();
    }

    out.flush().unwrap();
}

#[derive(Default)]
struct ReplState {
    src: String,
    quit: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReplCmd {
    Quit,
    Help,
    Run,
    Clear,
}

impl fmt::Display for ReplCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReplCmd::Quit => "quit the REPL",
                ReplCmd::Help => "display this message",
                ReplCmd::Run => "evaluate the REPL's full input",
                ReplCmd::Clear => "clear the REPL's state",
            }
        )
    }
}
