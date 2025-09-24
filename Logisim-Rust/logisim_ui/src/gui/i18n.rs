/// Complete internationalization system
/// Provides runtime language switching, string externalization, and locale support

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Spanish,
    French,
    German,
    Italian,
    Portuguese,
    Russian,
    Chinese,
    Japanese,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::German => "de",
            Language::Italian => "it",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Chinese => "zh",
            Language::Japanese => "ja",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Spanish => "Español",
            Language::French => "Français",
            Language::German => "Deutsch",
            Language::Italian => "Italiano",
            Language::Portuguese => "Português",
            Language::Russian => "Русский",
            Language::Chinese => "中文",
            Language::Japanese => "日本語",
        }
    }

    pub fn from_code(code: &str) -> Option<Language> {
        match code {
            "en" => Some(Language::English),
            "es" => Some(Language::Spanish),
            "fr" => Some(Language::French),
            "de" => Some(Language::German),
            "it" => Some(Language::Italian),
            "pt" => Some(Language::Portuguese),
            "ru" => Some(Language::Russian),
            "zh" => Some(Language::Chinese),
            "ja" => Some(Language::Japanese),
            _ => None,
        }
    }

    pub fn all_languages() -> Vec<Language> {
        vec![
            Language::English,
            Language::Spanish,
            Language::French,
            Language::German,
            Language::Italian,
            Language::Portuguese,
            Language::Russian,
            Language::Chinese,
            Language::Japanese,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct LocaleInfo {
    pub language: Language,
    pub country: Option<String>,
    pub variant: Option<String>,
}

impl LocaleInfo {
    pub fn new(language: Language) -> Self {
        Self {
            language,
            country: None,
            variant: None,
        }
    }

    pub fn with_country(mut self, country: String) -> Self {
        self.country = Some(country);
        self
    }

    pub fn with_variant(mut self, variant: String) -> Self {
        self.variant = Some(variant);
        self
    }

    pub fn to_string(&self) -> String {
        let mut result = self.language.code().to_string();
        if let Some(ref country) = self.country {
            result.push('_');
            result.push_str(country);
        }
        if let Some(ref variant) = self.variant {
            result.push('_');
            result.push_str(variant);
        }
        result
    }
}

type StringResources = HashMap<String, String>;

#[derive(Debug)]
pub struct I18nManager {
    current_language: Language,
    resources: HashMap<Language, StringResources>,
    fallback_language: Language,
}

impl Default for I18nManager {
    fn default() -> Self {
        let mut manager = Self {
            current_language: Language::English,
            resources: HashMap::new(),
            fallback_language: Language::English,
        };
        manager.load_default_resources();
        manager
    }
}

impl I18nManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    pub fn current_language(&self) -> &Language {
        &self.current_language
    }

    pub fn get_string(&self, key: &str) -> String {
        // Try current language first
        if let Some(resources) = self.resources.get(&self.current_language) {
            if let Some(value) = resources.get(key) {
                return value.clone();
            }
        }

        // Fall back to fallback language
        if let Some(resources) = self.resources.get(&self.fallback_language) {
            if let Some(value) = resources.get(key) {
                return value.clone();
            }
        }

        // Return key as fallback
        format!("[{}]", key)
    }

    pub fn get_string_with_args(&self, key: &str, args: &[&str]) -> String {
        let template = self.get_string(key);
        let mut result = template;
        
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, arg);
        }
        
        result
    }

    pub fn has_string(&self, key: &str) -> bool {
        if let Some(resources) = self.resources.get(&self.current_language) {
            if resources.contains_key(key) {
                return true;
            }
        }
        if let Some(resources) = self.resources.get(&self.fallback_language) {
            resources.contains_key(key)
        } else {
            false
        }
    }

    pub fn load_resources_from_string(&mut self, language: Language, content: &str) {
        let mut resources = StringResources::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim().to_string();
                resources.insert(key, value);
            }
        }
        
        self.resources.insert(language, resources);
    }

    fn load_default_resources(&mut self) {
        // Load English resources
        let en_resources = r#"
# Application strings
app.title=Logisim-RUST
app.version=Version {0}
app.about=Digital logic designer and simulator

# Menu items
menu.file=File
menu.edit=Edit
menu.view=View
menu.project=Project
menu.simulate=Simulate
menu.help=Help

menu.file.new=New
menu.file.open=Open...
menu.file.save=Save
menu.file.save_as=Save As...
menu.file.recent=Recent Files
menu.file.exit=Exit

menu.edit.undo=Undo
menu.edit.redo=Redo
menu.edit.cut=Cut
menu.edit.copy=Copy
menu.edit.paste=Paste
menu.edit.delete=Delete
menu.edit.select_all=Select All

menu.view.zoom_in=Zoom In
menu.view.zoom_out=Zoom Out
menu.view.zoom_fit=Fit to Window
menu.view.grid=Show Grid
menu.view.chronogram=Show Chronogram

# Toolbar
toolbar.select=Select
toolbar.wire=Wire
toolbar.input_pin=Input Pin
toolbar.output_pin=Output Pin
toolbar.and_gate=AND Gate
toolbar.or_gate=OR Gate
toolbar.not_gate=NOT Gate

# Component names
component.and_gate=AND Gate
component.or_gate=OR Gate
component.not_gate=NOT Gate
component.xor_gate=XOR Gate
component.nand_gate=NAND Gate
component.nor_gate=NOR Gate
component.xnor_gate=XNOR Gate
component.buffer=Buffer
component.d_flip_flop=D Flip-Flop
component.jk_flip_flop=JK Flip-Flop
component.sr_latch=SR Latch
component.d_latch=D Latch
component.register=Register
component.counter=Counter
component.adder=Adder
component.subtractor=Subtractor
component.multiplexer=Multiplexer
component.demultiplexer=Demultiplexer

# Properties
properties.title=Properties
properties.name=Name
properties.label=Label
properties.width=Bit Width
properties.inputs=Inputs
properties.trigger=Trigger
properties.facing=Facing

# Messages
message.file_saved=File saved successfully
message.file_loaded=File loaded successfully
message.error_loading_file=Error loading file: {0}
message.error_saving_file=Error saving file: {0}
message.unsaved_changes=You have unsaved changes. Save before closing?
message.confirm_delete=Are you sure you want to delete the selected components?

# Errors
error.invalid_file_format=Invalid file format
error.component_not_found=Component not found
error.connection_failed=Failed to create connection
error.simulation_error=Simulation error: {0}

# Chronogram
chronogram.title=Chronogram
chronogram.time=Time
chronogram.signal=Signal
chronogram.value=Value
chronogram.add_signal=Add Signal
chronogram.remove_signal=Remove Signal
chronogram.zoom_in=Zoom In
chronogram.zoom_out=Zoom Out
chronogram.export=Export
chronogram.clear=Clear

# Status messages
status.ready=Ready
status.simulating=Simulating...
status.loading=Loading...
status.saving=Saving...
status.components_selected={0} components selected
"#;
        self.load_resources_from_string(Language::English, en_resources);

        // Load Spanish resources
        let es_resources = r#"
# Application strings
app.title=Logisim-RUST
app.version=Versión {0}
app.about=Diseñador y simulador de lógica digital

# Menu items
menu.file=Archivo
menu.edit=Editar
menu.view=Ver
menu.project=Proyecto
menu.simulate=Simular
menu.help=Ayuda

menu.file.new=Nuevo
menu.file.open=Abrir...
menu.file.save=Guardar
menu.file.save_as=Guardar Como...
menu.file.recent=Archivos Recientes
menu.file.exit=Salir

menu.edit.undo=Deshacer
menu.edit.redo=Rehacer
menu.edit.cut=Cortar
menu.edit.copy=Copiar
menu.edit.paste=Pegar
menu.edit.delete=Eliminar
menu.edit.select_all=Seleccionar Todo

# Component names
component.and_gate=Puerta AND
component.or_gate=Puerta OR
component.not_gate=Puerta NOT
component.xor_gate=Puerta XOR
component.nand_gate=Puerta NAND
component.nor_gate=Puerta NOR
component.xnor_gate=Puerta XNOR
component.d_flip_flop=Flip-Flop D
component.register=Registro
component.counter=Contador
component.adder=Sumador

# Properties
properties.title=Propiedades
properties.name=Nombre
properties.label=Etiqueta
properties.width=Ancho de Bits
properties.inputs=Entradas

# Messages
message.file_saved=Archivo guardado exitosamente
message.file_loaded=Archivo cargado exitosamente
message.unsaved_changes=Tienes cambios sin guardar. ¿Guardar antes de cerrar?

# Status messages
status.ready=Listo
status.simulating=Simulando...
status.loading=Cargando...
status.saving=Guardando...
"#;
        self.load_resources_from_string(Language::Spanish, es_resources);

        // Load French resources
        let fr_resources = r#"
app.title=Logisim-RUST
app.version=Version {0}
app.about=Concepteur et simulateur de logique numérique

menu.file=Fichier
menu.edit=Édition
menu.view=Affichage
menu.project=Projet
menu.simulate=Simuler
menu.help=Aide

menu.file.new=Nouveau
menu.file.open=Ouvrir...
menu.file.save=Enregistrer
menu.file.save_as=Enregistrer Sous...
menu.file.exit=Quitter

component.and_gate=Porte ET
component.or_gate=Porte OU
component.not_gate=Porte NON
component.xor_gate=Porte OU-EX
component.register=Registre
component.counter=Compteur
component.adder=Additionneur

properties.title=Propriétés
properties.name=Nom
properties.label=Étiquette
properties.width=Largeur de Bits

status.ready=Prêt
status.simulating=Simulation...
status.loading=Chargement...
"#;
        self.load_resources_from_string(Language::French, fr_resources);

        // Add basic resources for other languages
        for language in [Language::German, Language::Italian, Language::Portuguese, Language::Russian, Language::Chinese, Language::Japanese] {
            let basic_resources = r#"
app.title=Logisim-RUST
menu.file=File
menu.edit=Edit
status.ready=Ready
"#;
            self.load_resources_from_string(language, basic_resources);
        }
    }

    pub fn detect_system_language() -> Language {
        if let Ok(locale) = std::env::var("LANG") {
            let lang_code = locale.split('_').next().unwrap_or("en");
            Language::from_code(lang_code).unwrap_or(Language::English)
        } else {
            Language::English
        }
    }
}

// Global instance for easy access
static I18N_INSTANCE: std::sync::OnceLock<Arc<RwLock<I18nManager>>> = std::sync::OnceLock::new();

pub fn get_i18n() -> Arc<RwLock<I18nManager>> {
    I18N_INSTANCE.get_or_init(|| {
        let mut manager = I18nManager::new();
        let system_lang = I18nManager::detect_system_language();
        manager.set_language(system_lang);
        Arc::new(RwLock::new(manager))
    }).clone()
}

// Convenience functions
pub fn tr(key: &str) -> String {
    get_i18n().read().unwrap().get_string(key)
}

pub fn tr_args(key: &str, args: &[&str]) -> String {
    get_i18n().read().unwrap().get_string_with_args(key, args)
}

pub fn set_language(language: Language) {
    get_i18n().write().unwrap().set_language(language);
}

pub fn current_language() -> Language {
    get_i18n().read().unwrap().current_language().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_codes() {
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::Spanish.code(), "es");
        assert_eq!(Language::from_code("fr"), Some(Language::French));
    }

    #[test]
    fn test_i18n_manager() {
        let mut manager = I18nManager::new();
        
        assert_eq!(manager.get_string("app.title"), "Logisim-RUST");
        
        manager.set_language(Language::Spanish);
        assert_eq!(manager.get_string("menu.file"), "Archivo");
        
        // Test fallback for nonexistent key - should return key in brackets
        assert!(manager.get_string("nonexistent.key").starts_with("["));
    }

    #[test]
    fn test_string_interpolation() {
        let manager = I18nManager::new();
        let result = manager.get_string_with_args("app.version", &["1.0.0"]);
        assert_eq!(result, "Version 1.0.0");
    }

    #[test]
    fn test_global_functions() {
        set_language(Language::English);
        assert_eq!(tr("app.title"), "Logisim-RUST");
        
        set_language(Language::Spanish);
        assert_eq!(tr("menu.file"), "Archivo");
    }
}