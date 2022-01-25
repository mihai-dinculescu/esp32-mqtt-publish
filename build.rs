use embuild::build::LinkArgs;

fn main() -> anyhow::Result<()> {
    // Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
    LinkArgs::output_propagated("ESP_IDF")?;

    Ok(())
}
