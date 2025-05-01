import { useParams } from "react-router";
import RecipeForm from "./RecipeForm";
import { useEffect, useState } from "react";
import { apiClient, MutateRecipeRequest } from "../../utils/api";

const EditRecipePage = () => {
  const { id } = useParams();
  const [recipe, setRecipe] = useState<MutateRecipeRequest>();

  useEffect(() => {
    const fetchRecipe = async () => {
      setRecipe((await apiClient.getRecipe(Number(id))) as MutateRecipeRequest);
    };
    fetchRecipe();
  }, [id]);

  return (
    recipe && (
      <>
        <h1>Edit Recipe</h1>
        <RecipeForm recipe={recipe} />
      </>
    )
  );
};

export default EditRecipePage;
