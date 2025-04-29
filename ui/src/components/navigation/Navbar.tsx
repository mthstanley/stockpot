import { Link } from "react-router";
import { useAuth } from "../auth/authContext";

const Navbar = () => {
  const auth = useAuth();

  return (
    <nav>
      <ul>
        <li>
          <Link to="/">Home</Link>
        </li>
        <li>
          <Link to="recipes">Recipes</Link>
        </li>
        {auth.user ? (
          <>
            <li>
              <Link to="users">Profile</Link>
            </li>
            <li>
              <Link to="users/signout">Sign-out</Link>
            </li>
          </>
        ) : (
          <>
            <li>
              <Link to="users/signin">Sign-in</Link>
            </li>
            <li>
              <Link to="users/signup">Sign-up</Link>
            </li>
          </>
        )}
      </ul>
    </nav>
  );
};

export default Navbar;
