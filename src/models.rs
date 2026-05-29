// models.rs

#[derive(Clone, Debug)]
pub struct CardProviderRange {
    pub range_start: String,
    pub range_end: String,
    pub provider_name: String,
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub card_number: String,
    pub card_provider: String,
    pub sum: u64,
}