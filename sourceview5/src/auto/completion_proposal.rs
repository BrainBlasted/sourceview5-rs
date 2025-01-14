// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct CompletionProposal(Interface<ffi::GtkSourceCompletionProposal>);

    match fn {
        get_type => || ffi::gtk_source_completion_proposal_get_type(),
    }
}

pub const NONE_COMPLETION_PROPOSAL: Option<&CompletionProposal> = None;

pub trait CompletionProposalExt: 'static {}

impl<O: IsA<CompletionProposal>> CompletionProposalExt for O {}

impl fmt::Display for CompletionProposal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CompletionProposal")
    }
}
