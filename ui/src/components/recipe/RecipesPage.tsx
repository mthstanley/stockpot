import { Link } from "react-router";
import RecipeList from "./RecipeList";

const RecipesPage = () => {
  return (
    <>
      <h1>Recipes</h1>
      <p>
        <Link to="create">Create New Recipe</Link>
      </p>
      <RecipeList />
    </>
  );
};

export default RecipesPage;
