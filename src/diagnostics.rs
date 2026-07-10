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
///         praise_term: String::from("pet"),
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
        $crate::diagnostics::Diagnostic {
            kind: $kind,
            labels: vec![
                $($crate::diagnostics::DiagnosticLabel { primary: true, msg: format!($primary_label_fmt $($primary_label_arg)*), range: $primary_label_span.into() },
                $($crate::diagnostics::DiagnosticLabel { primary: false, msg: format!($label_fmt $($label_arg)*), range: $label_span.into() }),*)?
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

#[allow(dead_code)]
#[derive(CompilerError, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ErrorKind {
    // Bug
    #[msg(
        "oh dear, {interp_title} made a mistake, why don't you be a good {praise_term} and go report it at `https://www.github.com/oughtum/bottomspeak/issues/` for me~"
    )]
    #[code(0000)]
    Bug {
        interp_title: String,
        praise_term: String,
    },

    #[msg(
        "sorry {petname}, {interp_title} doesn't need the character `{char}`, could you be ever such a good {praise_term} and remove it for me~"
    )]
    #[code(0001)]
    UnexpectedToken {
        petname: String,
        interp_title: String,
        praise_term: String,
        char: char,
    },

    #[msg(
        "I know speaking is hard for my good {praise_term} but could you please add a `{char_to_add}` at the end for me, {petname}~"
    )]
    #[code(0002)]
    UnfinishedEmoticon {
        praise_term: String,
        petname: String,
        char_to_add: char,
    },

    #[msg(
        "{interp_title} doesn't quite understand, did you mean to type one of these, {petname}? - `>w<`, `>//<`, `>.<`"
    )]
    #[code(0003)]
    AmbiguousUnfinishedEmoticon {
        interp_title: String,
        petname: String,
    },

    #[msg(
        "{interp_title} thinks your enthusiasm is adorable but keysmashes can't be longer than 127 characters, okay {petname}? mwah~"
    )]
    #[code(0004)]
    OverlongKeysmash {
        interp_title: String,
        petname: String,
    },

    #[msg("come on, be a good {praise_term} and use your words for {interp_title}~")]
    #[code(0005)]
    EmptySource {
        praise_term: String,
        interp_title: String,
    },

    #[msg(
        "subs need a name for their {interp_title} to call them by and so does this, can you add one for me {petname}?~"
    )]
    #[code(0006)]
    UnnamedSub {
        petname: String,
        interp_title: String,
    },

    #[msg(
        "{interp_title} isn't sure where you're trying to jump to, {petname}, could you be a good {praise_term} and add the name of the subroutine you want for me?~"
    )]
    #[code(0007)]
    UnnamedJump {
        interp_title: String,
        petname: String,
        praise_term: String,
    },

    #[msg(
        "all good subs need to communicate when they want {interp_title} to stop, so why don't you add a little `>.<` at the end for me~"
    )]
    #[code(0008)]
    SubWithoutReturn { interp_title: String },

    #[msg(
        "subroutines aren't allowed to be inside another subroutine, that's {interp_title}'s job~"
    )]
    #[code(0009)]
    NestedSubroutine { interp_title: String },

    #[msg(
        "oh you're a very talkative {praise_term} aren't you? unfortunately {interp_title}'s stack isn't infinite, so I think my adorable {praise_term} should shush now~"
    )]
    #[code(0010)]
    ReachedStackLimit {
        praise_term: String,
        interp_title: String,
    },

    #[msg(
        "oh {petname}, there aren't enough values on the stack to {op}, could you try again for {interp_title}?~"
    )]
    #[code(0011)]
    InsufficientElements {
        op: String,
        petname: String,
        interp_title: String,
    },

    #[msg(
        "sorry {petname}, you're trying to print 0x{codepoint:X} but unicode characters are in the range 0x000000-0x10FFFF, give it another go for {interp_title} okay?~"
    )]
    #[code(0012)]
    InvalidCodepoint {
        codepoint: u32,
        petname: String,
        interp_title: String,
    },

    #[msg(
        "{name} couldn't find any subroutine called `{interp_title}`, but I know you can do better for me next time {petname} <3"
    )]
    #[code(0013)]
    UnresolvedSubroutine {
        name: String,
        interp_title: String,
        petname: String,
    },

    #[msg(
        "{interp_title} needs some help understanding your code, {petname}, so be an obedient {praise_term} and add some comments~"
    )]
    #[code(0014)]
    UncommentedSource {
        interp_title: String,
        petname: String,
        praise_term: String,
    },

    #[msg(
        "{interp_title} has no input to use, {petname}, so be a good {praise_term} and provide some~"
    )]
    #[code(0015)]
    NoInput {
        interp_title: String,
        petname: String,
        praise_term: String,
    },
}
