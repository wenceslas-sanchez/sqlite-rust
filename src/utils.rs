use env_logger::{Builder, Target};

pub fn logger_builder() {
    Builder::new()
        .target(Target::Stdout)
        .init();
}
