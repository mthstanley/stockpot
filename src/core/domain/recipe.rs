use serde::Serialize;
use sqlx::{
    postgres::{PgHasArrayType, PgRow, PgTypeInfo},
    prelude::FromRow,
    Row,
};
use thiserror::Error;

use super::User;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected error occurred")]
    Unexpected,
    #[error("recipe with id `{0}` not found")]
    RecipeNotFound(i32),
}

#[derive(FromRow, Serialize, sqlx::Type, Debug, Clone, PartialEq)]
#[sqlx(type_name = "t_unit")]
pub struct Unit {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(FromRow, Serialize, sqlx::Type, Debug, Clone, PartialEq)]
#[sqlx(type_name = "t_ingredient")]
pub struct Ingredient {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(FromRow, Serialize, Debug, Clone, PartialEq)]
#[sqlx(type_name = "t_recipe_ingredient", transparent)]
pub struct RecipeIngredient {
    pub id: Option<i32>,
    pub recipe_id: Option<i32>,
    pub ingredient: Ingredient,
    pub quantity: i32,
    pub units: Unit,
    pub preparation: String,
}

impl PgHasArrayType for RecipeIngredient {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_t_recipe_ingredient")
    }
}

#[automatically_derived]
impl ::sqlx::encode::Encode<'_, ::sqlx::Postgres> for RecipeIngredient
where
    Option<i32>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Ingredient: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Ingredient: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Unit: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Unit: ::sqlx::types::Type<::sqlx::Postgres>,
    String: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn encode_by_ref(
        &self,
        buf: &mut ::sqlx::postgres::PgArgumentBuffer,
    ) -> ::sqlx::encode::IsNull {
        let mut encoder = ::sqlx::postgres::types::PgRecordEncoder::new(buf);
        encoder.encode(&self.id);
        encoder.encode(&self.recipe_id);
        encoder.encode(&self.ingredient);
        encoder.encode(&self.quantity);
        encoder.encode(&self.units);
        encoder.encode(&self.preparation);
        encoder.finish();
        ::sqlx::encode::IsNull::No
    }
    fn size_hint(&self) -> ::std::primitive::usize {
        6usize * (4 + 4)
            + <Option<i32> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.id)
            + <Option<i32> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.recipe_id)
            + <Ingredient as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.ingredient)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.quantity)
            + <Unit as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.units)
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.preparation)
    }
}
#[automatically_derived]
impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for RecipeIngredient
where
    Option<i32>: for<'q> ::sqlx::decode::Decode<'q, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: for<'q> ::sqlx::decode::Decode<'q, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    Ingredient: for<'q> ::sqlx::decode::Decode<'q, ::sqlx::Postgres>,
    Ingredient: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    Unit: for<'q> ::sqlx::decode::Decode<'q, ::sqlx::Postgres>,
    Unit: ::sqlx::types::Type<::sqlx::Postgres>,
    String: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn decode(
        value: ::sqlx::postgres::PgValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<Option<i32>>()?;
        let recipe_id = decoder.try_decode::<Option<i32>>()?;
        let ingredient = decoder.try_decode::<Ingredient>()?;
        let quantity = decoder.try_decode::<i32>()?;
        let units = decoder.try_decode::<Unit>()?;
        let preparation = decoder.try_decode::<String>()?;
        ::std::result::Result::Ok(RecipeIngredient {
            id,
            recipe_id,
            ingredient,
            quantity,
            units,
            preparation,
        })
    }
}
#[automatically_derived]
impl ::sqlx::Type<::sqlx::Postgres> for RecipeIngredient {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("t_recipe_ingredient")
    }
}

#[derive(FromRow, Serialize, Debug, PartialEq, Clone)]
#[sqlx(type_name = "t_step")]
pub struct Step {
    pub id: Option<i32>,
    pub recipe_id: Option<i32>,
    pub ordinal: i32,
    pub instruction: String,
}

impl PgHasArrayType for Step {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_t_step")
    }
}

#[automatically_derived]
impl ::sqlx::encode::Encode<'_, ::sqlx::Postgres> for Step
where
    Option<i32>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    String: for<'q> ::sqlx::encode::Encode<'q, ::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn encode_by_ref(
        &self,
        buf: &mut ::sqlx::postgres::PgArgumentBuffer,
    ) -> ::sqlx::encode::IsNull {
        let mut encoder = ::sqlx::postgres::types::PgRecordEncoder::new(buf);
        encoder.encode(&self.id);
        encoder.encode(&self.recipe_id);
        encoder.encode(&self.ordinal);
        encoder.encode(&self.instruction);
        encoder.finish();
        ::sqlx::encode::IsNull::No
    }
    fn size_hint(&self) -> ::std::primitive::usize {
        4usize * (4 + 4)
            + <Option<i32> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.id)
            + <Option<i32> as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.recipe_id)
            + <i32 as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.ordinal)
            + <String as ::sqlx::encode::Encode<::sqlx::Postgres>>::size_hint(&self.instruction)
    }
}
#[automatically_derived]
impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for Step
where
    Option<i32>: for<'q> ::sqlx::decode::Decode<'q, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<i32>: for<'q> ::sqlx::decode::Decode<'q, ::sqlx::Postgres>,
    Option<i32>: ::sqlx::types::Type<::sqlx::Postgres>,
    i32: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    String: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn decode(
        value: ::sqlx::postgres::PgValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<Option<i32>>()?;
        let recipe_id = decoder.try_decode::<Option<i32>>()?;
        let ordinal = decoder.try_decode::<i32>()?;
        let instruction = decoder.try_decode::<String>()?;
        ::std::result::Result::Ok(Step {
            id,
            recipe_id,
            ordinal,
            instruction,
        })
    }
}
#[automatically_derived]
impl ::sqlx::Type<::sqlx::Postgres> for Step {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("t_step")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Recipe {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub author: User,
    pub prep_time: Option<chrono::Duration>,
    pub cook_time: Option<chrono::Duration>,
    pub inactive_time: Option<chrono::Duration>,
    pub yield_quantity: i32,
    pub yield_units: Unit,
    pub ingredients: Vec<RecipeIngredient>,
    pub steps: Vec<Step>,
}

impl<'r> FromRow<'r, PgRow> for Recipe {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        let id: Option<i32> = row.try_get("id")?;
        let title: String = row.try_get("title")?;
        let description: Option<String> = row.try_get("description")?;
        let author: User = row.try_get("author")?;
        let prep_time_seconds: Option<i64> = row.try_get("prep_time")?;
        let prep_time = prep_time_seconds.map(|i| chrono::Duration::seconds(i));
        let cook_time_seconds: Option<i64> = row.try_get("cook_time")?;
        let cook_time = cook_time_seconds.map(|i| chrono::Duration::seconds(i));
        let inactive_time_seconds: Option<i64> = row.try_get("inactive_time")?;
        let inactive_time = inactive_time_seconds.map(|i| chrono::Duration::seconds(i));
        let yield_quantity: i32 = row.try_get("yield_quantity")?;
        let yield_units: Unit = row.try_get("yield_units")?;
        let ingredients: Vec<RecipeIngredient> = row.try_get("ingredients")?;
        let steps: Vec<Step> = row.try_get("steps")?;
        Ok(Recipe {
            id,
            title,
            description,
            author,
            prep_time,
            cook_time,
            inactive_time,
            yield_quantity,
            yield_units,
            ingredients,
            steps,
        })
    }
}
