use std::cmp::Ordering;

use codespan_reporting::{
    files::{self, Error, Files, line_starts},
    term::{
        self, Styles, StylesWriter,
        termcolor::{Color, ColorChoice, ColorSpec, StandardStream},
    },
};
use owo_colors::OwoColorize;

use crate::{diagnostic::Diagnostic, env::EnvVars};

pub(crate) struct SourceContext {
    pub(crate) source: String,
    pub(crate) name: String,
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) line_starts: Vec<usize>,
    pub(crate) env_vars: EnvVars,
}

impl SourceContext {
    pub(crate) fn new(source: &str, name: &str) -> crate::Result<Self> {
        let line_starts = line_starts(source).collect();

        Ok(Self {
            source: source.into(),
            name: name.into(),
            diagnostics: Vec::new(),
            line_starts,
            env_vars: EnvVars::new(),
        })
    }

    /// Reports a diagnostic.
    pub(crate) fn report(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    /// Samples a random interpreter name.
    pub(crate) fn rand_interp_title(&self) -> &str {
        self.env_vars.rand_interp_title()
    }

    /// Samples a random petname.
    pub(crate) fn rand_petname(&self) -> &str {
        self.env_vars.rand_petname()
    }

    /// Samples a random praise honorific.
    pub(crate) fn rand_praise_honorific(&self) -> &str {
        self.env_vars.rand_praise_honorific()
    }

    pub(crate) fn err_occurred(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Outputs the [`Diagnostic`](crate::diagnostics::Diagnostic)s of all modules to stderr.
    pub(crate) fn output_errors(&self) -> std::result::Result<(), files::Error> {
        let stream = StandardStream::stderr(ColorChoice::Auto);
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::Yellow));

        let style = Styles {
            secondary_label: spec.clone(),
            line_number: spec.clone(),
            source_border: spec.clone(),
            note_bullet: spec,
            ..Default::default()
        };
        let mut writer = StylesWriter::new(stream.lock(), &style);

        let config = term::Config {
            display_style: term::DisplayStyle::Rich,
            tab_width: 4,
            chars: term::Chars {
                snippet_start: "╭─".to_string(),
                source_border_left: '│',
                source_border_left_break: '│',
                note_bullet: '═',
                single_primary_caret: '^',
                single_secondary_caret: '─',
                multi_primary_caret_start: '^',
                multi_primary_caret_end: '^',
                multi_secondary_caret_start: '┅',
                multi_secondary_caret_end: '┅',
                multi_top_left: '╭',
                multi_top: '─',
                multi_bottom_left: '╰',
                multi_bottom: '─',
                multi_left: '│',
                pointer_left: '│',
            },
            start_context_lines: 3,
            end_context_lines: 3,
            before_label_lines: 2,
            after_label_lines: 2,
        };

        for diagnostic in self.diagnostics.iter() {
            term::emit_to_write_style(
                &mut writer,
                &config,
                self,
                &diagnostic.to_codespan_diagnostic(),
            )?;
        }

        let mut name = self.rand_interp_title().to_string();
        let name = format!("{}{}", name.remove(0).to_uppercase(), name);

        println!(
            "{}{}{}{}{}{}",
            name.magenta(),
            " found some errors in your code but it's okay, ".magenta(),
            self.rand_petname().magenta(),
            ", ".magenta(),
            self.rand_interp_title().magenta(),
            " believes in you <3\n".magenta(),
        );

        Ok::<(), files::Error>(())
    }

    fn line_start(&self, line_index: usize) -> Result<usize, Error> {
        match line_index.cmp(&self.line_starts.len()) {
            Ordering::Less => Ok(self.line_starts[line_index]),
            Ordering::Equal => Ok(self.source.len()),
            Ordering::Greater => Err(Error::LineTooLarge {
                given: line_index,
                max: self.line_starts.len() - 1,
            }),
        }
    }
}

impl<'a> Files<'a> for SourceContext {
    type FileId = usize;
    type Name = &'a str;
    type Source = &'a str;

    fn name(&'a self, _: Self::FileId) -> Result<Self::Name, Error> {
        Ok(&self.name)
    }

    fn source(&'a self, _: Self::FileId) -> Result<Self::Source, Error> {
        Ok(&self.source)
    }

    fn line_index(&'a self, _: Self::FileId, byte_index: usize) -> Result<usize, Error> {
        let line_starts = &self.line_starts;

        Ok(line_starts
            .binary_search(&byte_index)
            .unwrap_or_else(|next_line| next_line - 1))
    }

    fn line_range(
        &'a self,
        _: Self::FileId,
        line_index: usize,
    ) -> Result<std::ops::Range<usize>, Error> {
        let line_start = self.line_start(line_index)?;
        let next_line_start = self.line_start(line_index + 1)?;

        Ok(line_start..next_line_start)
    }
}
