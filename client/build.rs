use fs_extra::dir::CopyOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::remove_dir_all("assets/generated")?;
    std::fs::create_dir_all("assets/generated").unwrap();
    fs_extra::copy_items(
        &[
            "../protocol/target/assets/gods",
            "../protocol/target/assets/characters",
            "../protocol/target/assets/spells",
        ],
        "assets/generated",
        &CopyOptions::new().overwrite(true),
    )
    .unwrap();

    Ok(())
}
