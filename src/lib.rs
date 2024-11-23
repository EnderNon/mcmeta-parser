use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Mcmeta {
    pub pack: McmetaPack,
    pub features: McmetaFeatures,
    pub filter: Option<McmetaFilter>,
    pub overlays: Option<McmetaOverlays>,
    pub language: Option<Vec<McmetaLanguage>>
}
        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McmetaPack {
            pub description: Vec<McmetaPackDescription>,
            pub pack_format: u8,
            pub supported_formats: Option<McmetaPackSupportedformats>
        }
                #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                pub struct McmetaPackDescription {
                    pub text: String,
                    pub color: Option<String>
                }
                #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                pub struct McmetaPackSupportedformats {
                    pub min_inclusive: u8,
                    pub max_inclusive: u8
                }

        // I don't actually know wtf this looks like. I'm guessing based on minecraft wiki
        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McmetaFeatures {
            pub enabled: Vec<String>
        }

        // I don't what this looks like either
        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McmetaFilter {
            pub block: Vec<McmetaFilterBlock>
        }
                #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                pub struct McmetaFilterBlock {
                    pub Namespace: String,
                    pub Path: String
                }

        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McmetaOverlays {
            pub entries: Vec<McmetaOverlaysEntries>
        }

                #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                pub struct McmetaOverlaysEntries {
                    pub formats: McmetaOverlaysEntriesFormats,
                    pub directory: String
                }
                        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                        pub struct McmetaOverlaysEntriesFormats {
                            pub min_inclusive: u8,
                            pub max_inclusive: u8
                        }

        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McmetaLanguage {
            pub name: String,
            pub region: String,
            pub bidirectional: bool
        }