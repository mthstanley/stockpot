import { useEffect, useState } from "react";
import { apiClient, GetRecipeResponse } from "../../utils/api";
import RecipeCard from "./RecipeCard";

const RecipeList = () => {
  const [data, setData] = useState<Set<GetRecipeResponse>>(new Set());
  useEffect(() => {
    const fetchData = async () => {
      await apiClient.getRecipes().then((response) => {
        setData(response);
      });
    };
    fetchData();
  }, []);

  return (
    <>
      <ul>
        {[...data].map((recipe) => (
          <li>
            <RecipeCard recipe={recipe} headingLevel="h2" />
          </li>
        ))}
      </ul>
    </>
  );
};

export default RecipeList;
