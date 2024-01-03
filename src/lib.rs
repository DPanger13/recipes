uniffi::setup_scaffolding!();

#[derive(uniffi::Record)]
pub struct Home {
    pub total: i32,
    pub percentage: u8,
}

#[uniffi::export]
pub fn home() -> Home {
    Home {
        total: 8,
        percentage: 20,
    }
}
