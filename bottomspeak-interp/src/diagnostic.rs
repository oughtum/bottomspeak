use bottomspeak_macros::CompilerError;
use codespan_reporting::diagnostic::{
    Diagnostic as CodespanDiagnostic, Label, LabelStyle, Severity,
};
use core::range::Range;

/// Constructs a [`Diagnostic`].
///
/// # Examples
///
/// ```ignore
/// let diagnostic = diagnostic!(
///     ErrorKind::UnexpectedToken {
///         petname: String::from("sweetheart"),
///         interp_title: String::from("mommy"),
///         praise_honorific: String::from("pet"),
///         char: '%'
///     },
///     labels = [(10..11, "")],
/// );
/// ```
#[macro_export]
macro_rules! diagnostic {
    (
        $kind:expr
        $(,labels = [
            ($primary_label_span:expr, $primary_label_fmt:literal $($primary_label_arg:tt)*)
            $(,($label_span:expr, $label_fmt:literal $($label_arg:tt)*))*
        ])?
        $(,notes = [$(($notes_fmt:literal $($notes_arg:tt)*)),+])?$(,)?
    ) => {{
        $crate::diagnostic::Diagnostic {
            kind: $kind,
            labels: vec![
                $($crate::diagnostic::DiagnosticLabel { primary: true, msg: format!($primary_label_fmt $($primary_label_arg)*), range: $primary_label_span.into() },
                $($crate::diagnostic::DiagnosticLabel { primary: false, msg: format!($label_fmt $($label_arg)*), range: $label_span.into() }),*)?
            ],
            notes: vec![
                $($(format!($notes_fmt $($notes_arg)*)),*)?
            ]
        }
    }};
}

#[derive(Debug, Clone)]
pub(crate) struct Diagnostic {
    /// General information about the type of error encountered
    pub(crate) kind: ErrorKind,
    /// Labels providing extra context about the cause of the diagnostic
    pub(crate) labels: Vec<DiagnosticLabel>,
    /// Small snippets of information at the very bottom of the diagnostic
    /// that can be used to provide help
    pub(crate) notes: Vec<String>,
}

impl Diagnostic {
    pub(crate) fn to_codespan_diagnostic(&self) -> CodespanDiagnostic<usize> {
        let labels = self
            .labels
            .iter()
            .map(|label| {
                let style = if label.primary {
                    LabelStyle::Primary
                } else {
                    LabelStyle::Secondary
                };

                Label {
                    style,
                    file_id: 0,
                    range: label.range.into(),
                    message: label.msg.clone(),
                }
            })
            .collect();

        CodespanDiagnostic {
            severity: self.kind.severity(),
            code: Some(self.kind.code().to_string()),
            message: self.kind.msg(),
            labels,
            notes: self.notes.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DiagnosticLabel {
    /// Whether this label is the primary label
    pub(crate) primary: bool,
    /// The message this label displays
    pub(crate) msg: String,
    /// The range of source code this label spans
    pub(crate) range: Range<usize>,
}

pub(crate) trait CompilerError {
    fn msg(&self) -> String;
    fn code(&self) -> String;
    fn severity(&self) -> Severity;
}

#[derive(CompilerError, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ErrorKind {
    // Bug
    #[msg(
        "oh dear, {interp_title} made a mistake, why don't you be a good {praise_honorific} and go report it at `https://www.github.com/oughtum/bottomspeak-interp/issues/` for me~"
    )]
    #[code(0000)]
    Bug {
        interp_title: String,
        praise_honorific: String,
    },

    // Lexer
    #[msg(
        "sorry {petname}, {interp_title} doesn't need the character `{char}`, could you be ever such a good {praise_honorific} and remove it for me~"
    )]
    #[code(0001)]
    UnexpectedToken {
        petname: String,
        interp_title: String,
        praise_honorific: String,
        char: char,
    },

    #[msg(
        "you seem to have an unfinished emoticon here, could you please add a `{char_to_add}` at the end for me, {petname}~"
    )]
    #[code(0002)]
    UnfinishedEmoticon { petname: String, char_to_add: char },

    #[msg(
        "{interp_title} doesn't quite understand, did you mean to type one of these, {petname}? - `>w<`, `>//<`, `>.<`"
    )]
    #[code(0003)]
    AmbiguousUnfinishedEmoticon {
        interp_title: String,
        petname: String,
    },

    #[msg(
        "{interp_title} thinks your enthusiasm is adorable but keysmashes can't be longer than 255 characters, okay {petname}? mwah~"
    )]
    #[code(0004)]
    OverlongKeysmash {
        interp_title: String,
        petname: String,
    },

    /// Vm
    #[msg(
        "oh you're a very talkative {praise_honorific} aren't you? unfortunately {interp_title}'s stack isn't infinite, so I think my good little {praise_honorific} should shush now~"
    )]
    #[code(0005)]
    ReachedStackLimit {
        praise_honorific: String,
        interp_title: String,
    },

    #[msg(
        "oh {petname}, there aren't enough elements on the stack to {op}, could you try again for {interp_title}?~"
    )]
    #[code(0006)]
    InsufficientElements {
        op: String,
        petname: String,
        interp_title: String,
    },

    #[msg(
        "sorry {petname}, unicode characters are in the range 0x000000-0x10FFFF, git it another go for {interp_title} okay?~"
    )]
    #[code(0007)]
    InvalidCodepoint {
        petname: String,
        interp_title: String,
    },
}
