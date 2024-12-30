import { Link } from "react-router";
import { GetRecipeResponse } from "../../utils/api";

type HeadingLevel = "h1" | "h2" | "h3" | "h4" | "h5" | "h6";

const RecipeCard = ({
  recipe,
  headingLevel,
}: {
  recipe: GetRecipeResponse;
  headingLevel: HeadingLevel;
}) => {
  const Heading = headingLevel;
  return (
    <>
      <Heading>
        <Link to={`/recipes/${recipe.id}`}>{recipe.title}</Link>
      </Heading>
      <p>{recipe.description}</p>
    </>
  );
};

export default RecipeCard;
