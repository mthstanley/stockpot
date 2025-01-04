import { useFieldArray, useForm } from "react-hook-form";
import {
  apiClient,
  GetRecipeResponse,
  MutateRecipeRequest,
} from "../../utils/api";
import { useNavigate } from "react-router";
import { setEmptyOrStr, stripEmpty } from "../../utils/form";

const RecipeForm = ({ recipe }: { recipe?: MutateRecipeRequest }) => {
  const navigate = useNavigate();
  const { register, control, handleSubmit } = useForm<MutateRecipeRequest>({
    defaultValues: recipe,
  });
  const onSubmit = handleSubmit((recipe: MutateRecipeRequest) => {
    let response: Promise<GetRecipeResponse>;
    const strippedRecipe = stripEmpty(recipe);
    if (recipe.id !== null && recipe.id !== undefined) {
      response = apiClient.updateRecipe(strippedRecipe);
    } else {
      response = apiClient.createRecipe(strippedRecipe);
    }
    response.then((newRecipe) => navigate(`/recipes/${newRecipe.id}`));
  });

  const ingredients = useFieldArray({ control, name: "ingredients" });
  const steps = useFieldArray({ control, name: "steps" });
  const stepIndices: Record<number, number> = steps.fields.reduce(
    (map, { id }, index) => ({ ...map, [id]: index }),
    {},
  );

  return (
    <form onSubmit={onSubmit}>
      <fieldset>
        <legend>Details</legend>
        <p>
          <label htmlFor="title">Title</label>
          <input id="title" {...register("title", { required: true })} />
        </p>
        <p>
          <label htmlFor="description">Description</label>
          <input
            id="description"
            {...register("description", { setValueAs: setEmptyOrStr })}
          />
        </p>
        <p>
          <label htmlFor="prepTime">Prep Time</label>
          <input
            id="prepTime"
            type="number"
            {...register("prepTime", { valueAsNumber: true })}
          />
        </p>
        <p>
          <label htmlFor="cookTime">Cook Time</label>
          <input
            id="cookTime"
            type="number"
            {...register("cookTime", { valueAsNumber: true })}
          />
        </p>
        <p>
          <label htmlFor="inactiveTime">Inactive Time</label>
          <input
            id="inactiveTime"
            type="number"
            {...register("inactiveTime", { valueAsNumber: true })}
          />
        </p>
        <p>
          <label htmlFor="yieldQuantity">Yield Quantity</label>
          <input
            id="yieldQuantity"
            type="number"
            {...register("yieldQuantity", {
              required: true,
              valueAsNumber: true,
            })}
          />
        </p>
        <p>
          <label htmlFor="yieldUnits">Yield Units</label>
          <input
            id="yieldUnits"
            {...register("yieldUnits", {
              required: true,
              setValueAs: setEmptyOrStr,
            })}
          />
        </p>
      </fieldset>

      <fieldset>
        <legend>Ingredients</legend>
        <ul>
          {ingredients.fields.map((item, index) => (
            <li key={item.id}>
              <p>
                <label htmlFor={`ingredients.${index}.ingredient`}>
                  Ingredient
                </label>
                <input
                  id={`ingredients.${index}.ingredient`}
                  {...register(`ingredients.${index}.ingredient`, {
                    required: true,
                    setValueAs: setEmptyOrStr,
                  })}
                />
                <label htmlFor={`ingredients.${index}.quantity`}>
                  Quantity
                </label>
                <input
                  id={`ingredients.${index}.quantity`}
                  type="number"
                  {...register(`ingredients.${index}.quantity`, {
                    required: true,
                    valueAsNumber: true,
                  })}
                />
                <label htmlFor={`ingredients.${index}.units`}>Units</label>
                <input
                  id={`ingredients.${index}.units`}
                  {...register(`ingredients.${index}.units`, {
                    required: true,
                    setValueAs: setEmptyOrStr,
                  })}
                />
                <label htmlFor={`ingredients.${index}.preparation`}>
                  Preparation
                </label>
                <input
                  id={`ingredients.${index}.preparation`}
                  {...register(`ingredients.${index}.preparation`, {
                    setValueAs: setEmptyOrStr,
                  })}
                />
                <button type="button" onClick={() => ingredients.remove(index)}>
                  Delete
                </button>
              </p>
            </li>
          ))}
        </ul>
        <button
          type="button"
          onClick={() =>
            ingredients.append({
              ingredient: "",
              quantity: NaN,
              units: "",
              preparation: "",
            })
          }
        >
          Add Ingredient
        </button>
      </fieldset>

      <fieldset>
        <legend>Steps</legend>
        <ol>
          {steps.fields
            .sort((a, b) => a.ordinal - b.ordinal)
            .map((item, index) => (
              <li key={item.id}>
                <label htmlFor={`steps.${stepIndices[item.id]}.instruction`}>
                  Instruction
                </label>
                <input
                  id={`steps.${stepIndices[item.id]}.instruction`}
                  {...register(`steps.${stepIndices[item.id]}.instruction`, {
                    required: true,
                    setValueAs: setEmptyOrStr,
                  })}
                />
                {index === steps.fields.length - 1 && (
                  <button
                    type="button"
                    onClick={() => steps.remove(stepIndices[item.id])}
                  >
                    Delete
                  </button>
                )}
              </li>
            ))}
        </ol>
        <button
          type="button"
          onClick={() =>
            steps.append({ ordinal: steps.fields.length, instruction: "" })
          }
        >
          Add Step
        </button>
      </fieldset>

      <input type="submit" />
    </form>
  );
};

export default RecipeForm;
