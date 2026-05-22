#![feature(iter_intersperse)]

use std::{env, fs, io, path::PathBuf};

fn main() {
    let _ = bundle_js();
}

fn bundle_js() -> io::Result<()> {
    println!("cargo:rerun-if-changed=js/");

    let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to get CARGO_MANIFEST_DIR")
        .into();
    let js_dir = manifest_dir.join("js");

    let code = fs::read_dir(js_dir)?
        .filter_map(Result::ok)
        .filter_map(|f| {
            let content = fs::read_to_string(f.path()).ok()?;
            let is_macos = cfg!(target_os = "macos");

            if content.starts_with("// #TARGET MACOS") && !is_macos {
                None
            } else {
                Some(content)
            }
        })
        .intersperse("\n".to_string())
        .collect::<String>();

    let out_path = manifest_dir.join("target/scripts.js");

    fs::write(out_path, minify_js(&code))?;

    Ok(())
}

fn minify_js(code: &str) -> String {
    use std::sync::Arc;
    use swc::BoolOrDataConfig;
    use swc::config::JsMinifyOptions;
    use swc_common::{FileName, GLOBALS, Globals, SourceMap};

    let cm = Arc::new(SourceMap::default());
    let compiler = swc::Compiler::new(cm.clone());

    let fm = cm.new_source_file(Arc::new(FileName::Anon), code.to_string());

    GLOBALS.set(&Globals::new(), || {
        swc::try_with_handler(cm.clone(), Default::default(), |handler| {
            compiler.minify(
                fm,
                handler,
                &JsMinifyOptions {
                    compress: BoolOrDataConfig::from_bool(true),
                    mangle: BoolOrDataConfig::from_bool(true),
                    ..Default::default()
                },
                Default::default(),
            )
        })
        .expect("minification failed")
        .code
    })
}
