use dotenv::dotenv;

// ====================
// funtion
// ====================

pub fn set_default() {
    dotenv().ok();
}

// ====================
// const
// ====================

pub const SEPARATOR: &str = "---";