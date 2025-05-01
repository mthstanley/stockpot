import { Link, useParams } from "react-router";
import {
  apiClient,
  GetRecipeIngredientResponse,
  GetRecipeResponse,
  GetStepResponse,
} from "../../utils/api";
import { useEffect, useState } from "react";
import RecipeMeta from "./RecipeMeta";

const RecipeDetailPage = () => {
  const { id } = useParams();
  const [recipe, setRecipe] = useState<GetRecipeResponse>();

  useEffect(() => {
    const fetchRecipe = async () => {
      setRecipe(await apiClient.getRecipe(Number(id)));
    };
    fetchRecipe();
  }, [id]);

  return (
    recipe && (
      <>
        <article>
          <header>
            <h1>{recipe.title}</h1>
            <RecipeMeta recipe={recipe} />
            <p>{recipe.description}</p>
            <p>
              <Link to={`edit`}>Edit Recipe</Link>
            </p>
          </header>
          <section>
            <h2>Ingredients</h2>
            <ul>
              {recipe.ingredients.map(
                (ingredient: GetRecipeIngredientResponse, i: number) => (
                  <li key={i}>
                    {ingredient.quantity} {ingredient.units}{" "}
                    {ingredient.ingredient}, {ingredient.preparation}
                  </li>
                ),
              )}
            </ul>
          </section>
          <section>
            <h2>Steps</h2>
            <ol>
              {recipe.steps
                .sort((a, b) => a.ordinal - b.ordinal)
                .map((step: GetStepResponse, i: number) => (
                  <li key={i}>{step.instruction}</li>
                ))}
            </ol>
          </section>
        </article>
      </>
    )
  );
};

export default RecipeDetailPage;
