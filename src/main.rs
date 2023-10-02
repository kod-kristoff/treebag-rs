slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = Worksheet::new()?;

    // let ui_handle = ui.as_weak();
    ui.run()
}
