import { useParams } from "react-router";
import { apiClient, GetRecipeResponse } from "../../utils/api";
import { useEffect, useState } from "react";

const RecipeDetailPage = () => {
  const { id } = useParams();
  const [recipe, setRecipe] = useState<GetRecipeResponse>();

  useEffect(() => {
    const fetchRecipe = async () => {
      setRecipe(await apiClient.getRecipe(Number(id)));
    };
    fetchRecipe();
  }, [id]);

  return <h1>{recipe?.title}</h1>;
};

export default RecipeDetailPage;
