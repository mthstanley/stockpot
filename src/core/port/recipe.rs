use async_trait::async_trait;

use crate::core::domain;

#[async_trait]
pub trait RecipeRepository {
    async fn get_recipes(&self) -> Result<Vec<domain::Recipe>, domain::recipe::Error>;
    async fn get_recipe_by_id(&self, id: i32) -> Result<domain::Recipe, domain::recipe::Error>;
    async fn create_recipe(
        &self,
        recipe: domain::Recipe,
    ) -> Result<domain::Recipe, domain::recipe::Error>;
    async fn update_recipe(
        &self,
        recipe: domain::Recipe,
    ) -> Result<domain::Recipe, domain::recipe::Error>;
    async fn delete_recipe_by_id(&self, id: i32) -> Result<domain::Recipe, domain::recipe::Error>;
}

#[async_trait]
pub trait RecipeService {
    async fn get_recipes(&self) -> Result<Vec<domain::Recipe>, domain::recipe::Error>;
    async fn get_recipe_by_id(&self, id: i32) -> Result<domain::Recipe, domain::recipe::Error>;
    async fn create_recipe(
        &self,
        recipe: domain::Recipe,
    ) -> Result<domain::Recipe, domain::recipe::Error>;
    async fn update_recipe(
        &self,
        recipe: domain::Recipe,
    ) -> Result<domain::Recipe, domain::recipe::Error>;
    async fn delete_recipe_by_id(&self, id: i32) -> Result<domain::Recipe, domain::recipe::Error>;
}
