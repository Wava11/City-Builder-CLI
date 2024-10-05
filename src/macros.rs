#[macro_export]
macro_rules! tick {
    ( $timer:expr, $time:expr ) => {
        if !$timer.0.tick($time.delta()).just_finished() {
            return;
        }
    };
}
