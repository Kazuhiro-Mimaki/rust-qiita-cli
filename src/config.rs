use dotenv::dotenv;

pub fn set_default() {
    dotenv().ok();
}
