use crate::domain::category::Category;
use async_graphql::Object;

pub struct CategoryType(pub Category);

impl From<Category> for CategoryType {
    fn from(c: Category) -> Self {
        Self(c)
    }
}

#[Object]
impl CategoryType {
    async fn id(&self) -> &str {
        &self.0.id
    }

    async fn name(&self) -> &str {
        &self.0.name
    }

    async fn icon(&self) -> Option<&str> {
        self.0.icon.as_deref()
    }
}
