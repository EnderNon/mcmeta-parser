use pub serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
#[serde()]
pub struct McMeta {
    pub pack: McMeta_Pack,
    pub features: McMeta_Features,
    pub filter: Option<McMeta_Filter>,
    pub overlays: Option<McMeta_Overlays>,
    pub language: Option<Vec<McMeta_Language_Vec>>
}

        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McMeta_Pack {
            pub description: Vec<McMeta_Pack_Description_Vec>,
            pub pack_format: u8,
            pub supported_formats: Option<McMeta_Pack_SupportedFormats>
        }
                #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                pub struct McMeta_Pack_Description_Vec {
                    pub text: String,
                    pub color: Option<String>
                }
                #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                pub struct McMeta_Pack_SupportedFormats {
                    pub min_inclusive: u8,
                    pub max_inclusive: u8
                }

        // I don't actually know wtf this looks like. I'm guessing based on minecraft wiki
        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McMeta_Features {
            pub enabled: Vec<String>
        }

        // I don't what this looks like either
        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McMeta_Filter {
            pub block: Vec<McMeta_Filter_Block_Vec>
        }
                #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                pub struct McMeta_Filter_Block_Vec {
                    pub Namespace: String,
                    pub Path: String
                }

        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McMeta_Overlays {
            pub entries: Vec<McMeta_Overlays_Entries>
        }

                #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                pub struct McMeta_Overlays_Entries {
                    pub formats: McMeta_Overlays_Entries_Formats,
                    pub directory: String
                }
                #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
                pub struct McMeta_Overlays_Entries_Formats {
                    pub min_inclusive: u8,
                    pub max_inclusive: u8
                }

        #[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
        pub struct McMeta_Language_Vec {
            pub name: String,
            pub region: String,
            pub bidirectional: bool
        }