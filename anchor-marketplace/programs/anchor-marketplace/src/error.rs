use anchor_lang::error_code;

#[error_code]
pub enum MarketplaceError {
    #[msg("Name is too Long. Consider a shorter & meaningful name")]
    NameTooLong,
    #[msg("The collection is not valid")]
    InvalidCollection,
    #[msg("The collection is not verified")]
    CollectionNotVerified,
}