use crate::exercise::{CompiledExercise, Exercise, Mode, State};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};

// Verify that the provided container of Exercise objects
// can be compiled and run without any failures.
// Any such failures will be reported to the end user.
// If the Exercise being verified is a test, the verbose boolean
// determines whether or not the test harness outputs are displayed.
pub fn verify<'a>(
    exercises: &'a [Exercise],
    skip_n: usize,
    verbose: bool
) -> Result<(), &'a Exercise> {
    let bar = ProgressBar::new(exercises.len() as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("Progress: [{bar:60.green/red}] {pos}/{len}")
        .progress_chars("#>-")
    );
    bar.set_position(skip_n as u64);
    for exercise in exercises.iter().skip(skip_n) {
        let compile_result = match exercise.mode {
            Mode::Test => compile_and_test(&exercise, RunMode::Interactive, verbose),
            Mode::Compile => compile_and_run_interactively(&exercise),
            Mode::Clippy => compile_only(&exercise),
        };
        if !compile_result.unwrap_or(false) {
            return Err(exercise);
        }
        bar.inc(1);
    }
    Ok(())
}

enum RunMode {
    Interactive,
    NonInteractive,
}

// Compile and run the resulting test harness of the given Exercise
pub fn test(exercise: &Exercise, verbose: bool) -> Result<(), ()> {
    compile_and_test(exercise, RunMode::NonInteractive, verbose)?;
    Ok(())
}

// Invoke the rust compiler without running the resulting binary
fn compile_only(exercise: &Exercise) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {}...", exercise).as_str());
    progress_bar.enable_steady_tick(100);

    let _ = compile(&exercise, &progress_bar)?;
    progress_bar.finish_and_clear();

    Ok(prompt_for_completion(&exercise, None))
}

// Compile the given Exercise and run the resulting binary in an interactive mode
fn compile_and_run_interactively(exercise: &Exercise) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Compiling {}...", exercise).as_str());
    progress_bar.enable_steady_tick(100);

    let compilation = compile(&exercise, &progress_bar)?;

    progress_bar.set_message(format!("Running {}...", exercise).as_str());
    let result = compilation.run();
    progress_bar.finish_and_clear();

    let output = match result {
        Ok(output) => output,
        Err(output) => {
            warn!("Ran {} with errors", exercise);
            println!("{}", output.stdout);
            println!("{}", output.stderr);
            return Err(());
        }
    };

    Ok(prompt_for_completion(&exercise, Some(output.stdout)))
}

// Compile the given Exercise as a test harness and display
// the output if verbose is set to true
fn compile_and_test(exercise: &Exercise, run_mode: RunMode, verbose: bool) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("Testing {}...", exercise).as_str());
    progress_bar.enable_steady_tick(100);

    let compilation = compile(exercise, &progress_bar)?;
    let result = compilation.run();
    progress_bar.finish_and_clear();

    match result {
        Ok(output) => {
            if verbose {
                println!("{}", output.stdout);
            }
            if let RunMode::Interactive = run_mode {
                Ok(prompt_for_completion(&exercise, None))
            } else {
                Ok(true)
            }
        }
        Err(output) => {
            warn!(
                "Testing of {} failed! Please try again. Here's the output:",
                exercise
            );
            println!("{}", output.stdout);
            Err(())
        }
    }
}

// Compile the given Exercise and return an object with information
// about the state of the compilation
fn compile<'a, 'b>(
    exercise: &'a Exercise,
    progress_bar: &'b ProgressBar,
) -> Result<CompiledExercise<'a>, ()> {
    let compilation_result = exercise.compile();

    match compilation_result {
        Ok(compilation) => Ok(compilation),
        Err(output) => {
            progress_bar.finish_and_clear();
            warn!(
                "Compiling of {} failed! Please try again. Here's the output:",
                exercise
            );
            println!("{}", output.stderr);
            Err(())
        }
    }
}

fn prompt_for_completion(exercise: &Exercise, prompt_output: Option<String>) -> bool {
    let context = match exercise.state() {
        State::Done => return true,
        State::Pending(context) => context,
    };

    match exercise.mode {
        Mode::Compile => success!("Successfully ran {}!", exercise),
        Mode::Test => success!("Successfully tested {}!", exercise),
        Mode::Clippy => success!("Successfully compiled {}!", exercise),
    }

    let success_msg = match exercise.mode {
        Mode::Compile => "The code is compiling!",
        Mode::Test => "The code is compiling, and the tests pass!",
        Mode::Clippy => "The code is compiling, and 📎 Clippy 📎 is happy!",
    };

    println!();
    println!("🎉 🎉  {} 🎉 🎉", success_msg);
    println!();

    if let Some(output) = prompt_output {
        println!("Output:");
        println!("{}", separator());
        println!("{}", output);
        println!("{}", separator());
        println!();
    }

    println!("You can keep working on this exercise,");
    println!(
        "or jump into the next one by removing the {} comment:",
        style("`I AM NOT DONE`").bold()
    );
    println!();
    for context_line in context {
        let formatted_line = if context_line.important {
            format!("{}", style(context_line.line).bold())
        } else {
            context_line.line.to_string()
        };

        println!(
            "{:>2} {}  {}",
            style(context_line.number).blue().bold(),
            style("|").blue(),
            formatted_line
        );
    }

    false
}

fn separator() -> console::StyledObject<&'static str> {
    style("====================").bold()
}
