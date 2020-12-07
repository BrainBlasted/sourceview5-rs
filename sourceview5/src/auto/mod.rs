// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod buffer;
pub use self::buffer::BufferBuilder;
pub use self::buffer::BufferExt;
pub use self::buffer::{Buffer, NONE_BUFFER};

mod completion;
pub use self::completion::Completion;
pub use self::completion::CompletionBuilder;

mod completion_cell;
pub use self::completion_cell::CompletionCell;
pub use self::completion_cell::CompletionCellBuilder;

mod completion_context;
pub use self::completion_context::CompletionContext;
pub use self::completion_context::CompletionContextBuilder;

mod completion_proposal;
pub use self::completion_proposal::CompletionProposalExt;
pub use self::completion_proposal::{CompletionProposal, NONE_COMPLETION_PROPOSAL};

mod completion_provider;
pub use self::completion_provider::CompletionProviderExt;
pub use self::completion_provider::{CompletionProvider, NONE_COMPLETION_PROVIDER};

mod completion_snippets;
pub use self::completion_snippets::CompletionSnippetsBuilder;
pub use self::completion_snippets::CompletionSnippetsExt;
pub use self::completion_snippets::{CompletionSnippets, NONE_COMPLETION_SNIPPETS};

mod completion_words;
pub use self::completion_words::CompletionWordsBuilder;
pub use self::completion_words::CompletionWordsExt;
pub use self::completion_words::{CompletionWords, NONE_COMPLETION_WORDS};

mod file;
pub use self::file::FileBuilder;
pub use self::file::FileExt;
pub use self::file::{File, NONE_FILE};

mod file_loader;
pub use self::file_loader::FileLoader;
pub use self::file_loader::FileLoaderBuilder;

mod file_saver;
pub use self::file_saver::FileSaver;
pub use self::file_saver::FileSaverBuilder;

mod gutter;
pub use self::gutter::Gutter;
pub use self::gutter::GutterBuilder;

#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
mod gutter_lines;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
pub use self::gutter_lines::GutterLines;

mod gutter_renderer;
pub use self::gutter_renderer::GutterRendererExt;
pub use self::gutter_renderer::{GutterRenderer, NONE_GUTTER_RENDERER};

mod gutter_renderer_pixbuf;
pub use self::gutter_renderer_pixbuf::GutterRendererPixbufBuilder;
pub use self::gutter_renderer_pixbuf::GutterRendererPixbufExt;
pub use self::gutter_renderer_pixbuf::{GutterRendererPixbuf, NONE_GUTTER_RENDERER_PIXBUF};

mod gutter_renderer_text;
pub use self::gutter_renderer_text::GutterRendererTextBuilder;
pub use self::gutter_renderer_text::GutterRendererTextExt;
pub use self::gutter_renderer_text::{GutterRendererText, NONE_GUTTER_RENDERER_TEXT};

mod language;
pub use self::language::Language;

mod language_manager;
pub use self::language_manager::LanguageManager;
pub use self::language_manager::LanguageManagerBuilder;

mod map;
pub use self::map::MapBuilder;
pub use self::map::MapExt;
pub use self::map::{Map, NONE_MAP};

mod mark;
pub use self::mark::MarkBuilder;
pub use self::mark::MarkExt;
pub use self::mark::{Mark, NONE_MARK};

mod mark_attributes;
pub use self::mark_attributes::MarkAttributes;
pub use self::mark_attributes::MarkAttributesBuilder;

mod print_compositor;
pub use self::print_compositor::PrintCompositorBuilder;
pub use self::print_compositor::PrintCompositorExt;
pub use self::print_compositor::{PrintCompositor, NONE_PRINT_COMPOSITOR};

mod region;
pub use self::region::RegionBuilder;
pub use self::region::RegionExt;
pub use self::region::{Region, NONE_REGION};

mod search_context;
pub use self::search_context::SearchContext;
pub use self::search_context::SearchContextBuilder;

mod search_settings;
pub use self::search_settings::SearchSettingsBuilder;
pub use self::search_settings::SearchSettingsExt;
pub use self::search_settings::{SearchSettings, NONE_SEARCH_SETTINGS};

#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
mod snippet;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
pub use self::snippet::Snippet;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
pub use self::snippet::SnippetBuilder;

mod snippet_chunk;
pub use self::snippet_chunk::SnippetChunk;
pub use self::snippet_chunk::SnippetChunkBuilder;

mod snippet_context;
pub use self::snippet_context::SnippetContext;

#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
mod snippet_manager;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
pub use self::snippet_manager::SnippetManager;
#[cfg(any(feature = "v5_0", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v5_0")))]
pub use self::snippet_manager::SnippetManagerBuilder;

mod space_drawer;
pub use self::space_drawer::SpaceDrawer;
pub use self::space_drawer::SpaceDrawerBuilder;

mod style;
pub use self::style::Style;
pub use self::style::StyleBuilder;

mod style_scheme;
pub use self::style_scheme::StyleScheme;
pub use self::style_scheme::StyleSchemeBuilder;

mod style_scheme_chooser;
pub use self::style_scheme_chooser::StyleSchemeChooserExt;
pub use self::style_scheme_chooser::{StyleSchemeChooser, NONE_STYLE_SCHEME_CHOOSER};

mod style_scheme_chooser_button;
pub use self::style_scheme_chooser_button::{
    StyleSchemeChooserButton, NONE_STYLE_SCHEME_CHOOSER_BUTTON,
};

mod style_scheme_chooser_widget;
pub use self::style_scheme_chooser_widget::{
    StyleSchemeChooserWidget, NONE_STYLE_SCHEME_CHOOSER_WIDGET,
};

mod style_scheme_manager;
pub use self::style_scheme_manager::StyleSchemeManager;
pub use self::style_scheme_manager::StyleSchemeManagerBuilder;

mod tag;
pub use self::tag::TagBuilder;
pub use self::tag::TagExt;
pub use self::tag::{Tag, NONE_TAG};

mod view;
pub use self::view::ViewBuilder;
pub use self::view::ViewExt;
pub use self::view::{View, NONE_VIEW};

mod encoding;
pub use self::encoding::Encoding;

mod enums;
pub use self::enums::BackgroundPatternType;
pub use self::enums::BracketMatchType;
pub use self::enums::ChangeCaseType;
pub use self::enums::CompletionActivation;
pub use self::enums::CompletionColumn;
pub use self::enums::CompressionType;
pub use self::enums::FileLoaderError;
pub use self::enums::FileSaverError;
pub use self::enums::GutterRendererAlignmentMode;
pub use self::enums::NewlineType;
pub use self::enums::SmartHomeEndType;
pub use self::enums::ViewGutterPosition;

mod flags;
pub use self::flags::FileSaverFlags;
pub use self::flags::SortFlags;
pub use self::flags::SpaceLocationFlags;
pub use self::flags::SpaceTypeFlags;

#[doc(hidden)]
pub mod traits {
    pub use super::BufferExt;
    pub use super::CompletionProposalExt;
    pub use super::CompletionProviderExt;
    pub use super::CompletionSnippetsExt;
    pub use super::CompletionWordsExt;
    pub use super::FileExt;
    pub use super::GutterRendererExt;
    pub use super::GutterRendererPixbufExt;
    pub use super::GutterRendererTextExt;
    pub use super::MapExt;
    pub use super::MarkExt;
    pub use super::PrintCompositorExt;
    pub use super::RegionExt;
    pub use super::SearchSettingsExt;
    pub use super::StyleSchemeChooserExt;
    pub use super::TagExt;
    pub use super::ViewExt;
}
