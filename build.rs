fn main() {
    slint_build::compile_with_config(
        "ui/app.slint",
        slint_build::CompilerConfiguration::new()
            // .with_sdf_fonts(true)
            // .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer)
            .with_style("cosmic-dark".into()),
    )
    .unwrap();
}
