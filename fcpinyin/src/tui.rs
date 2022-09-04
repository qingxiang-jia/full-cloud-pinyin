use cursive::views::TextView;

pub fn run_tui() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(TextView::new(
        "This will be a test bed for the input method. Press <q> to quit.",
    ));

    siv.run();
}
