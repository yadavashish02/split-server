use async_graphql::Object;
use crate::domain::currency::Currency;

pub struct CurrencyType(pub Currency);

impl From<Currency> for CurrencyType {
    fn from(c: Currency) -> Self {
        Self(c)
    }
}

#[Object]
impl CurrencyType {
    async fn code(&self) -> &str {
        &self.0.code
    }

    async fn name(&self) -> &str {
        &self.0.name
    }

    async fn symbol(&self) -> &str {
        &self.0.symbol
    }

    async fn minor_unit(&self) -> i32 {
        self.0.minor_unit
    }
}
