use serde::{Serialize,Deserialize};
use std::collections::HashMap;
/// The root object.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Mcmeta {
    /// Holds the pack information.
    pub pack: McmetaPack,
    /// (optional) Section for selecting experimental features.
    pub features: Option<McmetaFeatures>,
    /// (optional) Section for filtering out files from packs applied below this one.
    pub filter: Option<McmetaFilter>,
    /// (optional) Section for specifying the overlays, which are sub-packs applied over the "normal" contents of a pack.
    pub overlays: Option<McmetaOverlays>,
    /// (optional) Only present in resource packs — Contains additional languages to add to the language menu.
    pub language: Option<HashMap<String,McmetaLanguage>>
}
        /// Holds the pack information.
        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McmetaPack {
            /// A raw JSON text that appears when hovering over the pack's name in the list given by the `/datapack list` command, or when viewing the pack in the Create World screen.
            pub description: Vec<McmetaPackDescription>,
            /// Determines the version(s) of Minecraft that this pack is compatible with.
            ///
            /// See <https://minecraft.wiki/w/Pack_format> for a full list of pack format numbers.
            pub pack_format: u8,
            /// (optional) Describes a range for pack formats that this pack supports.
            ///
            /// The range has to include the value of [`McmetaPack::pack_format`].
            pub supported_formats: Option<McmetaPackSupportedformats>
        }
                /// Determines the version(s) of Minecraft that this pack is compatible with.
                ///
                /// See <https://minecraft.wiki/w/Pack_format> for a full list of pack format numbers.
                #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                pub struct McmetaPackDescription {
                    /// The text in the description.
                    pub text: String,
                    /// The colour of the text in the description.
                    pub color: Option<String>
                }
                /// (optional) Describes a range for pack formats that this pack supports.
                ///
                /// The range has to include the value of [`McmetaPack::pack_format`].
                #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                pub struct McmetaPackSupportedformats {
                    /// Minimum Pack format number
                    pub min_inclusive: u8,
                    /// Maximum pack format number
                    pub max_inclusive: u8
                }

        // I don't what this looks like
        /// (optional) Section for selecting experimental features.
        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McmetaFeatures {
            /// List of enabled feature flags.
            ///
            /// Each item in the Vec is a location of a feature flag.
            pub enabled: Vec<String>
        }

        // I don't what this looks like
        /// (optional) Section for filtering out files from packs applied below this one.
        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McmetaFilter {
            /// Vector of patterns.
            ///
            /// Any file that matches one of the patterns inside [`McmetaFilter::block`] is treated as if it was not present in the pack at all.
            pub block: Vec<McmetaFilterBlock>
        }
                /// A pattern.
                ///
                /// Any file that matches one of the patterns inside here is treated as if it was not present in the pack at all.
                #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                pub struct McmetaFilterBlock {
                    /// A regular expression for the namespace of files to be filtered out. If unspecified, it applies to every namespace.
                    pub namespace: String,
                    /// A regular expression for the paths of files to be filtered out. If unspecified, it applies to every file.
                    pub path: String
                }

        // I don't what this looks like
        /// (optional) Section for specifying the overlays, which are sub-packs applied over the "normal" contents of a pack.
        ///
        /// Their directories have their own assets and data directories, and are placed in the pack's root directory.
        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McmetaOverlays {
            /// Vector of overlays. The order is important, as the first in the list is applied first.
            pub entries: Vec<McmetaOverlaysEntries>
        }
                #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                /// An overlay.
                pub struct McmetaOverlaysEntries {
                    /// Describes a range for pack formats when this overlay should be active.
                    pub formats: McmetaOverlaysEntriesFormats,
                    /// The directory to overlay for the respective versions (allowed characters: a-z, 0-9, _ and -). In this directory, pack.mcmeta and pack.png are ignored.
                    pub directory: String
                }
                        /// Describes a range for pack formats when this overlay should be active.
                        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                        pub struct McmetaOverlaysEntriesFormats {
                            /// Minimum Pack format number.
                            pub min_inclusive: u8,
                            /// Maximum Pack format number.
                            pub max_inclusive: u8
                        }

        // I don't what this looks like
        /// (optional) Only present in resource packs — Contains additional languages to add to the language menu.
        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McmetaLanguage {
            /// The full name of the language
            pub name: String,
            /// The country or region name
            pub region: String,
            /// If true, the language reads right to left.
            pub bidirectional: bool
        }