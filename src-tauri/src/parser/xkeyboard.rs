// Copy from https://github.com/pop-os/distinst/blob/8322c936a91ba5812ac9cbef79282a04bad738ea/crates/locales/src/keyboard_layout.rs#L82

use eyre::Result;
use serde::{Deserialize, Serialize};
use serde_xml_rs as xml;
use std::{fs::File, io::BufReader};

/// A keyboard layout, flattened for consumption by the frontend.
#[derive(Debug, Clone, Serialize)]
pub struct KeyboardLayoutInfo {
    pub name: String,
    pub description: String,
    pub variants: Vec<KeyboardVariantInfo>,
}

/// A variant of a keyboard layout, flattened for consumption by the frontend.
#[derive(Debug, Clone, Serialize)]
pub struct KeyboardVariantInfo {
    pub name: String,
    pub description: String,
}

/// Check whether the given string is a syntactically valid XKB layout or
/// variant name. This is used to sanitise values before they are passed to
/// external commands.
pub fn is_valid_name(name: &str) -> bool {
    let mut bytes = name.bytes();

    matches!(bytes.next(), Some(b) if b.is_ascii_alphanumeric())
        && name.len() <= 64
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

/// A list of keyboard layouts parsed from `/usr/share/X11/xkb/rules/base.xml`.
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyboardLayouts {
    #[serde(rename = "layoutList")]
    pub layout_list: LayoutList,
}

impl KeyboardLayouts {
    /// Fetch the layouts from the layout list.
    pub fn get_layouts(&self) -> &[KeyboardLayout] {
        &self.layout_list.layout
    }

    /// Flatten the layout list into a structure convenient for the frontend.
    ///
    /// Layouts and variants with names that cannot be safely passed to
    /// `setxkbmap` or written to configuration files are dropped.
    pub fn to_info(&self) -> Vec<KeyboardLayoutInfo> {
        self.get_layouts()
            .iter()
            .filter(|layout| is_valid_name(layout.get_name()))
            .map(|layout| KeyboardLayoutInfo {
                name: layout.get_name().to_string(),
                description: layout.get_description().to_string(),
                variants: layout
                    .get_variants()
                    .map(|variants| {
                        variants
                            .iter()
                            .filter(|variant| is_valid_name(variant.get_name()))
                            .map(|variant| KeyboardVariantInfo {
                                name: variant.get_name().to_string(),
                                description: variant.get_description().to_string(),
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
            })
            .collect()
    }
}

/// A list of keyboard layouts.
#[derive(Debug, Deserialize, Serialize)]
pub struct LayoutList {
    pub layout: Vec<KeyboardLayout>,
}

/// A keyboard layout, which contains an optional list of variants, a name, and a description.
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyboardLayout {
    #[serde(rename = "configItem")]
    pub config_item: ConfigItem,
    #[serde(rename = "variantList")]
    pub variant_list: Option<VariantList>,
}

impl KeyboardLayout {
    /// Fetches the name of the keyboard layout.
    pub fn get_name(&self) -> &str {
        &self.config_item.name
    }

    /// Fetches a description of the layout.
    pub fn get_description(&self) -> &str {
        &self.config_item.description
    }

    /// Fetches a list of possible layout variants.
    pub fn get_variants(&self) -> Option<&Vec<KeyboardVariant>> {
        self.variant_list.as_ref().and_then(|x| x.variant.as_ref())
    }
}

/// Contains the name and description of a keyboard layout.
#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigItem {
    pub name: String,
    #[serde(rename = "shortDescription")]
    pub short_description: Option<String>,
    pub description: String,
}

/// A list of possible variants of a keyboard layout.
#[derive(Debug, Deserialize, Serialize)]
pub struct VariantList {
    pub variant: Option<Vec<KeyboardVariant>>,
}

/// A variant of a keyboard layout.
#[derive(Debug, Deserialize, Serialize)]
pub struct KeyboardVariant {
    #[serde(rename = "configItem")]
    pub config_item: ConfigItem,
}

impl KeyboardVariant {
    /// The name of this variant of a keybaord layout.
    pub fn get_name(&self) -> &str {
        &self.config_item.name
    }

    /// A description of this variant of a keyboard layout.
    pub fn get_description(&self) -> &str {
        &self.config_item.description
    }
}

const X11_BASE_RULES: &str = "/usr/share/X11/xkb/rules/base.xml";

/// Fetches a list of keyboard layouts from `/usr/share/X11/xkb/rules/base.xml`.
pub fn get_keyboard_layouts() -> Result<KeyboardLayouts> {
    Ok(xml::from_reader(BufReader::new(File::open(
        X11_BASE_RULES,
    )?))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<xkbConfigRegistry version="2.43">
<layoutList>
  <layout>
    <configItem>
      <name>us</name>
      <shortDescription>en</shortDescription>
      <description>English (US)</description>
    </configItem>
    <variantList>
      <variant>
        <configItem>
          <name>dvorak</name>
          <description>English (Dvorak)</description>
        </configItem>
      </variant>
      <variant>
        <configItem>
          <name>colemak</name>
          <description>English (Colemak)</description>
        </configItem>
      </variant>
    </variantList>
  </layout>
  <layout>
    <configItem>
      <name>fr</name>
      <shortDescription>fr</shortDescription>
      <description>French</description>
    </configItem>
  </layout>
  <layout>
    <configItem>
      <name>bad;name</name>
      <description>Invalid layout</description>
    </configItem>
  </layout>
</layoutList>
</xkbConfigRegistry>
"#;

    #[test]
    fn parses_layouts_and_variants() {
        let layouts = xml::from_reader::<KeyboardLayouts, _>(Cursor::new(SAMPLE)).unwrap();
        let info = layouts.to_info();

        assert_eq!(info.len(), 2);
        assert!(!info.iter().any(|x| x.name == "bad;name"));

        let us = info.iter().find(|x| x.name == "us").unwrap();
        assert_eq!(us.description, "English (US)");
        assert_eq!(us.variants.len(), 2);
        assert!(us.variants.iter().any(|x| x.name == "dvorak"));

        let fr = info.iter().find(|x| x.name == "fr").unwrap();
        assert!(fr.variants.is_empty());
    }

    #[test]
    fn validates_names() {
        assert!(is_valid_name("us"));
        assert!(is_valid_name("dvorak"));
        assert!(is_valid_name("us-intl"));

        assert!(!is_valid_name(""));
        assert!(!is_valid_name("us; rm -rf /"));
        assert!(!is_valid_name("-layout"));
        assert!(!is_valid_name(&"a".repeat(65)));
    }

    #[test]
    fn parses_system_base_xml() {
        if !std::path::Path::new(X11_BASE_RULES).exists() {
            return;
        }

        let layouts = get_keyboard_layouts().unwrap();
        let info = layouts.to_info();

        assert!(info.iter().any(|x| x.name == "us"));
        assert!(info
            .iter()
            .find(|x| x.name == "us")
            .unwrap()
            .variants
            .iter()
            .any(|x| x.name == "dvorak"));
    }
}
