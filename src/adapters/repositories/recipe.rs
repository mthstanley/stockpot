use crate::core::{domain, port};
use async_trait::async_trait;
use sqlx::{Postgres, QueryBuilder};

pub struct PostgresRecipeRepository {
    db_pool: sqlx::postgres::PgPool,
}

impl PostgresRecipeRepository {
    pub fn new(db_pool: sqlx::postgres::PgPool) -> PostgresRecipeRepository {
        PostgresRecipeRepository { db_pool }
    }
}

#[async_trait]
impl port::RecipeRepository for PostgresRecipeRepository {
    async fn get_recipes(&self) -> Result<Vec<domain::Recipe>, domain::recipe::Error> {
        sqlx::query_as(
            r#"
            SELECT
                r.id as id,
                r.title as title,
                r.description as description,
                (au.id, au.name)::t_app_user as author,
                EXTRACT(EPOCH FROM r.prep_time)::bigint as prep_time,
                EXTRACT(EPOCH FROM r.cook_time)::bigint as cook_time,
                EXTRACT(EPOCH FROM r.inactive_time)::bigint as inactive_time,
                r.yield_quantity as yield_quantity,
                (ru.id, ru.name)::t_unit as yield_units,
                array(SELECT (s.id, s.recipe, s.ordinal, s.instruction)::t_step FROM step s WHERE s.recipe = r.id) as steps,
                array(
                    SELECT
                    (
                        ri.id,
                        ri.recipe,
                        (i.id, i.name)::t_ingredient,
                        ri.quantity,
                        (riu.id, riu.name)::t_unit,
                        ri.preparation
                    )::t_recipe_ingredient
                    FROM recipe_ingredient ri
                    JOIN ingredient i ON i.id = ri.ingredient
                    JOIN unit riu ON ri.units = riu.id
                    WHERE ri.recipe = r.id
                ) as ingredients
            FROM
                recipe AS r
                JOIN app_user au ON r.author = au.id
                JOIN unit ru ON r.yield_units = ru.id;
            "#,
        )
            .fetch_all(&self.db_pool)
            .await
            .map_err(|e| {
                log::error!("Failed to find recipes due to: {}", e);
                domain::recipe::Error::Unexpected
            })
    }

    async fn get_recipe_by_id(&self, id: i32) -> Result<domain::Recipe, domain::recipe::Error> {
        sqlx::query_as(
            r#"
            SELECT
                r.id as id,
                r.title as title,
                r.description as description,
                (au.id, au.name)::t_app_user as author,
                EXTRACT(EPOCH FROM r.prep_time)::bigint as prep_time,
                EXTRACT(EPOCH FROM r.cook_time)::bigint as cook_time,
                EXTRACT(EPOCH FROM r.inactive_time)::bigint as inactive_time,
                r.yield_quantity as yield_quantity,
                (ru.id, ru.name)::t_unit as yield_units,
                array(SELECT (s.id, s.recipe, s.ordinal, s.instruction)::t_step FROM step s WHERE s.recipe = r.id) as steps,
                array(
                    SELECT
                    (
                        ri.id,
                        ri.recipe,
                        (i.id, i.name)::t_ingredient,
                        ri.quantity,
                        (riu.id, riu.name)::t_unit,
                        ri.preparation
                    )::t_recipe_ingredient
                    FROM recipe_ingredient ri
                    JOIN ingredient i ON i.id = ri.ingredient
                    JOIN unit riu ON ri.units = riu.id
                    WHERE ri.recipe = r.id
                ) as ingredients
            FROM
                recipe AS r
                JOIN app_user au ON r.author = au.id
                JOIN unit ru ON r.yield_units = ru.id
            WHERE r.id = $1;
            "#,
        ).bind(id)
            .fetch_one(&self.db_pool)
            .await.map_err(|e| {
                match e {
                    sqlx::Error::RowNotFound => domain::recipe::Error::RecipeNotFound(id),
                    _ => {
                        log::error!("Failed to find recipe by id `{}` due to: {}", id, e);
                        domain::recipe::Error::Unexpected
                    }
                }
            })
    }

    async fn create_recipe(
        &self,
        recipe: domain::Recipe,
    ) -> Result<domain::Recipe, domain::recipe::Error> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            WITH i_ins_ingredient AS (
                INSERT INTO
                    ingredient (name)
            "#,
        );
        query_builder.push_values(recipe.ingredients.to_vec(), |mut b, i| {
            b.push_bind(i.ingredient.name);
        });
        query_builder.push(
            r#"
             ON CONFLICT (name) DO NOTHING RETURNING * ),
            i_ingredient AS (
                SELECT * FROM i_ins_ingredient
                UNION ALL
                SELECT * FROM ingredient WHERE name in (
            "#,
        );
        let mut sep = query_builder.separated(", ");
        for recipe_ingredient in recipe.ingredients.to_vec() {
            sep.push_bind(recipe_ingredient.ingredient.name);
        }
        query_builder.push(")),");

        query_builder.push(
            r#"
            i_recipe AS (
                INSERT INTO recipe (
                    title,
                    description,
                    author,
                    prep_time,
                    cook_time,
                    inactive_time,
                    yield_quantity,
                    yield_units
                )
                VALUES (
            "#,
        );
        let mut sep = query_builder.separated(", ");
        sep.push_bind(&recipe.title)
            .push_bind(&recipe.description)
            .push("(SELECT id FROM app_user WHERE name = ")
            .push_bind_unseparated(&recipe.author.name)
            .push_unseparated(")")
            .push_bind(recipe.prep_time)
            .push_bind(recipe.cook_time)
            .push_bind(recipe.inactive_time)
            .push_bind(recipe.yield_quantity)
            .push("(SELECT id FROM unit WHERE name = ")
            .push_bind_unseparated(&recipe.yield_units.name)
            .push_unseparated(")");
        query_builder.push(" ) RETURNING *),");

        query_builder.push(
            r#"
            i_step AS (
                INSERT INTO step (recipe, ordinal, instruction)
            "#,
        );
        query_builder.push_values(recipe.steps.to_vec(), |mut b, step| {
            b.push("(SELECT id FROM i_recipe)")
                .push_bind(step.ordinal)
                .push_bind(step.instruction);
        });
        query_builder.push(" RETURNING *),");

        query_builder.push(
            r#"
            i_recipe_ingredient AS (
                INSERT INTO recipe_ingredient (recipe, ingredient, quantity, units, preparation)
                VALUES
            "#,
        );
        let mut sep = query_builder.separated(", ");
        for recipe_ingredient in recipe.ingredients.to_vec() {
            sep.push("(")
                .push_unseparated("(SELECT id FROM i_recipe)")
                .push("(SELECT id FROM i_ingredient WHERE name = ")
                .push_bind_unseparated(recipe_ingredient.ingredient.name)
                .push_unseparated(")")
                .push_bind(recipe_ingredient.quantity)
                .push("(SELECT id FROM unit WHERE name = ")
                .push_bind_unseparated(recipe_ingredient.units.name)
                .push_unseparated(")")
                .push_bind(recipe_ingredient.preparation)
                .push_unseparated(")");
        }
        query_builder.push(" RETURNING *)");
        query_builder.push(r#"
            SELECT
                r.id as id,
                r.title as title,
                r.description as description,
                (au.id, au.name)::t_app_user as author,
                EXTRACT(EPOCH FROM r.prep_time)::bigint as prep_time,
                EXTRACT(EPOCH FROM r.cook_time)::bigint as cook_time,
                EXTRACT(EPOCH FROM r.inactive_time)::bigint as inactive_time,
                r.yield_quantity as yield_quantity,
                (ru.id, ru.name)::t_unit as yield_units,
                array(SELECT (s.id, s.recipe, s.ordinal, s.instruction)::t_step FROM i_step s) as steps,
                array(
                    SELECT
                    (
                        ri.id,
                        ri.recipe,
                        (i.id, i.name)::t_ingredient,
                        ri.quantity,
                        (riu.id, riu.name)::t_unit,
                        ri.preparation
                    )::t_recipe_ingredient
                    FROM i_recipe_ingredient ri
                    JOIN i_ingredient i ON i.id = ri.ingredient
                    JOIN unit riu ON ri.units = riu.id
                ) as ingredients
            FROM
                i_recipe AS r
                JOIN app_user au ON r.author = au.id
                JOIN unit ru ON r.yield_units = ru.id;
            "#);

        query_builder
            .build_query_as()
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| {
                log::error!("Failed to create recipe {:?} due to: {}", recipe, e);
                domain::recipe::Error::Unexpected
            })
    }

    async fn update_recipe(
        &self,
        recipe: domain::Recipe,
    ) -> Result<domain::Recipe, domain::recipe::Error> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            WITH i_ins_ingredient AS (
                INSERT INTO
                    ingredient (name)
            "#,
        );
        query_builder.push_values(&recipe.ingredients, |mut b, i| {
            b.push_bind(&i.ingredient.name);
        });
        query_builder.push(
            r#"
             ON CONFLICT (name) DO NOTHING RETURNING * ),
            i_ingredient AS (
                SELECT * FROM i_ins_ingredient
                UNION ALL
                SELECT * FROM ingredient WHERE name in (
            "#,
        );
        let mut sep = query_builder.separated(", ");
        for recipe_ingredient in &recipe.ingredients {
            sep.push_bind(&recipe_ingredient.ingredient.name);
        }
        query_builder.push(")),");

        query_builder.push(
            r#"
            i_recipe AS (
                INSERT INTO recipe (
                    id,
                    title,
                    description,
                    author,
                    prep_time,
                    cook_time,
                    inactive_time,
                    yield_quantity,
                    yield_units
                )
                VALUES (
            "#,
        );
        let mut sep = query_builder.separated(", ");
        sep.push_bind(recipe.id)
            .push_bind(&recipe.title)
            .push_bind(&recipe.description)
            .push("(SELECT id FROM app_user WHERE name = ")
            .push_bind_unseparated(&recipe.author.name)
            .push_unseparated(")")
            .push_bind(recipe.prep_time)
            .push_bind(recipe.cook_time)
            .push_bind(recipe.inactive_time)
            .push_bind(recipe.yield_quantity)
            .push("(SELECT id FROM unit WHERE name = ")
            .push_bind_unseparated(&recipe.yield_units.name)
            .push_unseparated(")");
        query_builder.push(
            r#"
            ) ON CONFLICT (id) DO UPDATE SET
            title = EXCLUDED.title,
            description = EXCLUDED.description,
            author = EXCLUDED.author,
            prep_time = EXCLUDED.prep_time,
            cook_time = EXCLUDED.cook_time,
            inactive_time = EXCLUDED.inactive_time,
            yield_quantity = EXCLUDED.yield_quantity,
            yield_units = EXCLUDED.yield_units
            RETURNING *),
            "#,
        );

        query_builder.push(
            r#"
            i_step AS (
                INSERT INTO step (id, recipe, ordinal, instruction)
            "#,
        );
        query_builder.push_values(&recipe.steps, |mut b, step| {
            if let Some(id) = step.id {
                b.push_bind(id);
            } else {
                b.push("DEFAULT");
            }
            b.push("(SELECT id FROM i_recipe)")
                .push_bind(step.ordinal)
                .push_bind(&step.instruction);
        });
        query_builder.push(
            r#"
             ON CONFLICT (id) DO UPDATE SET
            recipe = EXCLUDED.recipe,
            ordinal = EXCLUDED.ordinal,
            instruction = EXCLUDED.instruction
            RETURNING *),
            d_step AS (
                DELETE FROM step s USING i_recipe WHERE recipe = i_recipe.id AND s.id NOT IN (SELECT id FROM i_step)
            ),
            "#,
        );

        query_builder.push(
            r#"
            i_recipe_ingredient AS (
                INSERT INTO recipe_ingredient (id, recipe, ingredient, quantity, units, preparation)
                VALUES
            "#,
        );
        let mut sep = query_builder.separated(", ");
        for recipe_ingredient in &recipe.ingredients {
            sep.push("(");
            if let Some(id) = recipe_ingredient.id {
                sep.push_bind_unseparated(id);
            } else {
                sep.push_unseparated("DEFAULT");
            }
            sep.push("(SELECT id FROM i_recipe)")
                .push("(SELECT id FROM i_ingredient WHERE name = ")
                .push_bind_unseparated(&recipe_ingredient.ingredient.name)
                .push_unseparated(")")
                .push_bind(recipe_ingredient.quantity)
                .push("(SELECT id FROM unit WHERE name = ")
                .push_bind_unseparated(&recipe_ingredient.units.name)
                .push_unseparated(")")
                .push_bind(&recipe_ingredient.preparation)
                .push_unseparated(")");
        }
        query_builder.push(
            r#"
             ON CONFLICT (id) DO UPDATE SET
            recipe = EXCLUDED.recipe,
            ingredient = EXCLUDED.ingredient,
            quantity = EXCLUDED.quantity,
            units = EXCLUDED.units,
            preparation = EXCLUDED.preparation
            RETURNING *),
            d_recipe_ingredient AS (
                DELETE FROM recipe_ingredient ri
                USING i_recipe
                WHERE recipe = i_recipe.id AND ri.id NOT IN (
                    SELECT id FROM i_recipe_ingredient
                )
            )
            "#,
        );
        query_builder.push(r#"
            SELECT
                r.id as id,
                r.title as title,
                r.description as description,
                (au.id, au.name)::t_app_user as author,
                EXTRACT(EPOCH FROM r.prep_time)::bigint as prep_time,
                EXTRACT(EPOCH FROM r.cook_time)::bigint as cook_time,
                EXTRACT(EPOCH FROM r.inactive_time)::bigint as inactive_time,
                r.yield_quantity as yield_quantity,
                (ru.id, ru.name)::t_unit as yield_units,
                array(SELECT (s.id, s.recipe, s.ordinal, s.instruction)::t_step FROM i_step s) as steps,
                array(
                    SELECT
                    (
                        ri.id,
                        ri.recipe,
                        (i.id, i.name)::t_ingredient,
                        ri.quantity,
                        (riu.id, riu.name)::t_unit,
                        ri.preparation
                    )::t_recipe_ingredient
                    FROM i_recipe_ingredient ri
                    JOIN i_ingredient i ON i.id = ri.ingredient
                    JOIN unit riu ON ri.units = riu.id
                ) as ingredients
            FROM
                i_recipe AS r
                JOIN app_user au ON r.author = au.id
                JOIN unit ru ON r.yield_units = ru.id;
            "#);

        query_builder
            .build_query_as()
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| {
                log::error!("Failed to update recipe {:?} due to: {}", recipe, e);
                domain::recipe::Error::Unexpected
            })
    }

    async fn delete_recipe_by_id(&self, id: i32) -> Result<domain::Recipe, domain::recipe::Error> {
        let recipe = self.get_recipe_by_id(id).await?;
        sqlx::query("DELETE FROM recipe WHERE id = $1")
            .bind(id)
            .execute(&self.db_pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => domain::recipe::Error::RecipeNotFound(id),
                _ => {
                    log::error!("Failed to delete recipe by id `{}` due to: {}", id, e);
                    domain::recipe::Error::Unexpected
                }
            })?;
        return Ok(recipe);
    }
}
