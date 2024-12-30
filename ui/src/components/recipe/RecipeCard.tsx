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
    <section>
      <Heading>
        <Link to={`/recipes/${recipe.id}`}>{recipe.title}</Link>
      </Heading>
      <dl>
        {recipe.prepTime !== null && (
          <>
            <dt>Prep Time</dt>
            <dd>{recipe.prepTime} seconds</dd>
          </>
        )}
        {recipe.cookTime !== null && (
          <>
            <dt>Cook Time</dt>
            <dd>{recipe.cookTime} seconds</dd>
          </>
        )}
        {recipe.inactiveTime !== null && (
          <>
            <dt>Inactive Time</dt>
            <dd>{recipe.inactiveTime} seconds</dd>
          </>
        )}
        <dt>Yields</dt>
        <dd>
          {recipe.yieldQuantity} {recipe.yieldUnits}
        </dd>
      </dl>
      <p>{recipe.description}</p>
    </section>
  );
};

export default RecipeCard;
