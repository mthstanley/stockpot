import { Link } from "react-router";

const HomePage = () => {
  return (
    <>
      <h1>Stockpot</h1>
      <Link to="users">Profile</Link> <Link to="users/signup">Sign-up</Link>{" "}
      <Link to="users/signin">Sign-in</Link> <Link to="recipes">Recipes</Link>
    </>
  );
};

export default HomePage;
