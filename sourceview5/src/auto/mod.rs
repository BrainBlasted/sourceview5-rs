// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

mod buffer;
pub use self::buffer::{Buffer, BufferClass, NONE_BUFFER};
pub use self::buffer::BufferExt;
pub use self::buffer::BufferBuilder;

mod completion;
pub use self::completion::{Completion, CompletionClass, NONE_COMPLETION};
pub use self::completion::CompletionExt;
pub use self::completion::CompletionBuilder;

mod completion_cell;
pub use self::completion_cell::{CompletionCell, CompletionCellClass, NONE_COMPLETION_CELL};
pub use self::completion_cell::CompletionCellExt;
pub use self::completion_cell::CompletionCellBuilder;

mod completion_context;
pub use self::completion_context::{CompletionContext, CompletionContextClass, NONE_COMPLETION_CONTEXT};
pub use self::completion_context::CompletionContextExt;
pub use self::completion_context::CompletionContextBuilder;

mod completion_proposal;
pub use self::completion_proposal::{CompletionProposal, NONE_COMPLETION_PROPOSAL};
pub use self::completion_proposal::CompletionProposalExt;

mod completion_provider;
pub use self::completion_provider::{CompletionProvider, NONE_COMPLETION_PROVIDER};
pub use self::completion_provider::CompletionProviderExt;

mod completion_snippets;
pub use self::completion_snippets::{CompletionSnippets, CompletionSnippetsClass, NONE_COMPLETION_SNIPPETS};
pub use self::completion_snippets::CompletionSnippetsExt;
pub use self::completion_snippets::CompletionSnippetsBuilder;

mod completion_words;
pub use self::completion_words::{CompletionWords, CompletionWordsClass, NONE_COMPLETION_WORDS};
pub use self::completion_words::CompletionWordsExt;
pub use self::completion_words::CompletionWordsBuilder;

mod file;
pub use self::file::{File, FileClass, NONE_FILE};
pub use self::file::FileExt;
pub use self::file::FileBuilder;

mod file_loader;
pub use self::file_loader::{FileLoader, FileLoaderClass, NONE_FILE_LOADER};
pub use self::file_loader::FileLoaderExt;
pub use self::file_loader::FileLoaderBuilder;

mod file_saver;
pub use self::file_saver::{FileSaver, FileSaverClass, NONE_FILE_SAVER};
pub use self::file_saver::FileSaverExt;
pub use self::file_saver::FileSaverBuilder;

mod gutter;
pub use self::gutter::{Gutter, GutterClass, NONE_GUTTER};
pub use self::gutter::GutterExt;
pub use self::gutter::GutterBuilder;

mod gutter_lines;
pub use self::gutter_lines::{GutterLines, GutterLinesClass, NONE_GUTTER_LINES};
pub use self::gutter_lines::GutterLinesExt;

mod gutter_renderer;
pub use self::gutter_renderer::{GutterRenderer, GutterRendererClass, NONE_GUTTER_RENDERER};
pub use self::gutter_renderer::GutterRendererExt;

mod gutter_renderer_pixbuf;
pub use self::gutter_renderer_pixbuf::{GutterRendererPixbuf, GutterRendererPixbufClass, NONE_GUTTER_RENDERER_PIXBUF};
pub use self::gutter_renderer_pixbuf::GutterRendererPixbufExt;
pub use self::gutter_renderer_pixbuf::GutterRendererPixbufBuilder;

mod gutter_renderer_text;
pub use self::gutter_renderer_text::{GutterRendererText, GutterRendererTextClass, NONE_GUTTER_RENDERER_TEXT};
pub use self::gutter_renderer_text::GutterRendererTextExt;
pub use self::gutter_renderer_text::GutterRendererTextBuilder;

mod language;
pub use self::language::{Language, LanguageClass, NONE_LANGUAGE};
pub use self::language::LanguageExt;

mod language_manager;
pub use self::language_manager::{LanguageManager, LanguageManagerClass, NONE_LANGUAGE_MANAGER};
pub use self::language_manager::LanguageManagerExt;
pub use self::language_manager::LanguageManagerBuilder;

mod map;
pub use self::map::{Map, MapClass, NONE_MAP};
pub use self::map::MapExt;
pub use self::map::MapBuilder;

mod mark;
pub use self::mark::{Mark, MarkClass, NONE_MARK};
pub use self::mark::MarkExt;
pub use self::mark::MarkBuilder;

mod mark_attributes;
pub use self::mark_attributes::{MarkAttributes, MarkAttributesClass, NONE_MARK_ATTRIBUTES};
pub use self::mark_attributes::MarkAttributesExt;
pub use self::mark_attributes::MarkAttributesBuilder;

mod print_compositor;
pub use self::print_compositor::{PrintCompositor, PrintCompositorClass, NONE_PRINT_COMPOSITOR};
pub use self::print_compositor::PrintCompositorExt;
pub use self::print_compositor::PrintCompositorBuilder;

mod region;
pub use self::region::{Region, RegionClass, NONE_REGION};
pub use self::region::RegionExt;
pub use self::region::RegionBuilder;

mod search_context;
pub use self::search_context::{SearchContext, SearchContextClass, NONE_SEARCH_CONTEXT};
pub use self::search_context::SearchContextExt;
pub use self::search_context::SearchContextBuilder;

mod search_settings;
pub use self::search_settings::{SearchSettings, SearchSettingsClass, NONE_SEARCH_SETTINGS};
pub use self::search_settings::SearchSettingsExt;
pub use self::search_settings::SearchSettingsBuilder;

#[cfg(any(feature = "v5_0", feature = "dox"))]
mod snippet;
#[cfg(any(feature = "v5_0", feature = "dox"))]
pub use self::snippet::{Snippet, SnippetClass, NONE_SNIPPET};
#[cfg(any(feature = "v5_0", feature = "dox"))]
pub use self::snippet::SnippetExt;
#[cfg(any(feature = "v5_0", feature = "dox"))]
pub use self::snippet::SnippetBuilder;

mod snippet_chunk;
pub use self::snippet_chunk::{SnippetChunk, SnippetChunkClass, NONE_SNIPPET_CHUNK};
pub use self::snippet_chunk::SnippetChunkExt;
pub use self::snippet_chunk::SnippetChunkBuilder;

mod snippet_context;
pub use self::snippet_context::{SnippetContext, SnippetContextClass, NONE_SNIPPET_CONTEXT};
pub use self::snippet_context::SnippetContextExt;

#[cfg(any(feature = "v5_0", feature = "dox"))]
mod snippet_manager;
#[cfg(any(feature = "v5_0", feature = "dox"))]
pub use self::snippet_manager::{SnippetManager, SnippetManagerClass, NONE_SNIPPET_MANAGER};
#[cfg(any(feature = "v5_0", feature = "dox"))]
pub use self::snippet_manager::SnippetManagerExt;
#[cfg(any(feature = "v5_0", feature = "dox"))]
pub use self::snippet_manager::SnippetManagerBuilder;

mod space_drawer;
pub use self::space_drawer::{SpaceDrawer, SpaceDrawerClass, NONE_SPACE_DRAWER};
pub use self::space_drawer::SpaceDrawerExt;
pub use self::space_drawer::SpaceDrawerBuilder;

mod style;
pub use self::style::{Style, StyleClass, NONE_STYLE};
pub use self::style::StyleExt;
pub use self::style::StyleBuilder;

mod style_scheme;
pub use self::style_scheme::{StyleScheme, StyleSchemeClass, NONE_STYLE_SCHEME};
pub use self::style_scheme::StyleSchemeExt;
pub use self::style_scheme::StyleSchemeBuilder;

mod style_scheme_chooser;
pub use self::style_scheme_chooser::{StyleSchemeChooser, NONE_STYLE_SCHEME_CHOOSER};
pub use self::style_scheme_chooser::StyleSchemeChooserExt;

mod style_scheme_chooser_button;
pub use self::style_scheme_chooser_button::{StyleSchemeChooserButton, StyleSchemeChooserButtonClass, NONE_STYLE_SCHEME_CHOOSER_BUTTON};

mod style_scheme_chooser_widget;
pub use self::style_scheme_chooser_widget::{StyleSchemeChooserWidget, StyleSchemeChooserWidgetClass, NONE_STYLE_SCHEME_CHOOSER_WIDGET};

mod style_scheme_manager;
pub use self::style_scheme_manager::{StyleSchemeManager, StyleSchemeManagerClass, NONE_STYLE_SCHEME_MANAGER};
pub use self::style_scheme_manager::StyleSchemeManagerExt;
pub use self::style_scheme_manager::StyleSchemeManagerBuilder;

mod tag;
pub use self::tag::{Tag, TagClass, NONE_TAG};
pub use self::tag::TagExt;
pub use self::tag::TagBuilder;

mod view;
pub use self::view::{View, ViewClass, NONE_VIEW};
pub use self::view::ViewExt;
pub use self::view::ViewBuilder;

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
    pub use super::CompletionExt;
    pub use super::CompletionCellExt;
    pub use super::CompletionContextExt;
    pub use super::CompletionProposalExt;
    pub use super::CompletionProviderExt;
    pub use super::CompletionSnippetsExt;
    pub use super::CompletionWordsExt;
    pub use super::FileExt;
    pub use super::FileLoaderExt;
    pub use super::FileSaverExt;
    pub use super::GutterExt;
    pub use super::GutterLinesExt;
    pub use super::GutterRendererExt;
    pub use super::GutterRendererPixbufExt;
    pub use super::GutterRendererTextExt;
    pub use super::LanguageExt;
    pub use super::LanguageManagerExt;
    pub use super::MapExt;
    pub use super::MarkExt;
    pub use super::MarkAttributesExt;
    pub use super::PrintCompositorExt;
    pub use super::RegionExt;
    pub use super::SearchContextExt;
    pub use super::SearchSettingsExt;
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    pub use super::SnippetExt;
    pub use super::SnippetChunkExt;
    pub use super::SnippetContextExt;
    #[cfg(any(feature = "v5_0", feature = "dox"))]
    pub use super::SnippetManagerExt;
    pub use super::SpaceDrawerExt;
    pub use super::StyleExt;
    pub use super::StyleSchemeExt;
    pub use super::StyleSchemeChooserExt;
    pub use super::StyleSchemeManagerExt;
    pub use super::TagExt;
    pub use super::ViewExt;
}
