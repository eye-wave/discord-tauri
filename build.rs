#![feature(iter_intersperse)]

use std::{env, fs, io, path::PathBuf};

fn main() {
    let _ = bundle_js();
}

#[derive(PartialEq)]
enum TargetOs {
    Macos,
    Linux,
    Window,
    Unknown,
}

impl From<&str> for TargetOs {
    fn from(value: &str) -> Self {
        match value {
            "MACOS" => Self::Macos,
            "LINUX" => Self::Linux,
            "WINDOWS" => Self::Window,
            _ => Self::Unknown,
        }
    }
}

fn bundle_js() -> io::Result<()> {
    println!("cargo:rerun-if-changed=js/");

    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to get CARGO_MANIFEST_DIR")
        .into();
    let js_dir = manifest_dir.join("js");

    let target = if cfg!(target_os = "macos") {
        TargetOs::Macos
    } else if cfg!(target_os = "windows") {
        TargetOs::Window
    } else if cfg!(target_os = "linux") {
        TargetOs::Linux
    } else {
        TargetOs::Unknown
    };

    let code = fs::read_dir(js_dir)?
        .filter_map(Result::ok)
        .filter_map(|f| {
            let content = fs::read_to_string(f.path()).ok()?;

            if is_target(&target, &content) {
                None
            } else {
                Some(content)
            }
        })
        .intersperse("\n".to_string())
        .collect::<String>();

    let out_path = manifest_dir.join("target").join("scripts.js");

    fs::write(out_path, minify_js(&code))?;

    Ok(())
}

fn is_target(target: &TargetOs, code: &str) -> bool {
    const PRAGMA: &str = "// #TARGET ";

    let start = PRAGMA.len();
    let src_os: TargetOs = code[start..start + 7]
        .find('\n')
        .map(|i| &code[start..start + i])
        .unwrap_or_default()
        .into();

    *target == src_os
}

fn minify_js(code: &str) -> String {
    use std::sync::Arc;
    use swc::BoolOrDataConfig;
    use swc::config::{IsModule, JsMinifyOptions};
    use swc_common::{FileName, GLOBALS, Globals, SourceMap};
    use swc_ecma_minifier::option::terser::{TerserCompressorOptions, TerserTopLevelOptions};

    let cm = Arc::new(SourceMap::default());
    let compiler = swc::Compiler::new(cm.clone());

    let fm = cm.new_source_file(Arc::new(FileName::Anon), code.to_string());

    GLOBALS.set(&Globals::new(), || {
        swc::try_with_handler(cm.clone(), Default::default(), |handler| {
            compiler.minify(
                fm,
                handler,
                &JsMinifyOptions {
                    compress: BoolOrDataConfig::from_obj(TerserCompressorOptions {
                        toplevel: Some(TerserTopLevelOptions::Bool(false)),
                        top_retain: None,
                        ..Default::default()
                    }),
                    mangle: BoolOrDataConfig::from_obj(swc_ecma_minifier::option::MangleOptions {
                        top_level: Some(false),
                        ..Default::default()
                    }),
                    module: IsModule::Bool(false),
                    ..Default::default()
                },
                Default::default(),
            )
        })
        .expect("minification failed")
        .code
    })
}
