use std::path::PathBuf;

use crate::unreal::Module;

/// Context needed to rename an Unreal Engine module.
pub struct Context {
    /// The root of the project that the module is part of.
    pub project_root: PathBuf,
    /// The name of the project.
    pub project_name: String,
    /// Build targets for the project that the module is part of.
    pub project_targets: Vec<PathBuf>,
    /// The specific module to rename.
    pub target_module: Module,
    /// The target name for the module.
    pub target_name: String,
    /// The source file that includes the module implement macro.
    pub source_with_implement_macro: Option<PathBuf>,
    /// Header files that include the module export macro.
    pub headers_with_export_macro: Vec<PathBuf>,
}
