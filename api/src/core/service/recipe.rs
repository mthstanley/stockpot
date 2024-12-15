use crate::core::{domain, port};
use async_trait::async_trait;

pub struct DefaultRecipeService {
    recipe_repository: Box<dyn port::RecipeRepository + Send + Sync>,
}

impl DefaultRecipeService {
    pub fn new(
        recipe_repository: Box<dyn port::RecipeRepository + Send + Sync>,
    ) -> DefaultRecipeService {
        DefaultRecipeService { recipe_repository }
    }
}

#[async_trait]
impl port::RecipeService for DefaultRecipeService {
    async fn get_recipes(&self) -> Result<Vec<domain::Recipe>, domain::recipe::Error> {
        Ok(self.recipe_repository.get_recipes().await?)
    }
    async fn get_recipe_by_id(&self, id: i32) -> Result<domain::Recipe, domain::recipe::Error> {
        Ok(self.recipe_repository.get_recipe_by_id(id).await?)
    }
    async fn create_recipe(
        &self,
        recipe: domain::Recipe,
    ) -> Result<domain::Recipe, domain::recipe::Error> {
        Ok(self.recipe_repository.create_recipe(recipe).await?)
    }
    async fn update_recipe(
        &self,
        recipe: domain::Recipe,
    ) -> Result<domain::Recipe, domain::recipe::Error> {
        Ok(self.recipe_repository.update_recipe(recipe).await?)
    }
    async fn delete_recipe_by_id(&self, id: i32) -> Result<domain::Recipe, domain::recipe::Error> {
        Ok(self.recipe_repository.delete_recipe_by_id(id).await?)
    }
}
