pub struct Home {
    pub total: i32,
    pub percentage: u8,
}

pub fn home() -> Home {
    Home {
        total: 8,
        percentage: 20,
    }
}
