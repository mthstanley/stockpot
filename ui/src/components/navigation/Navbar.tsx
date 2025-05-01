import { NavLink } from "react-router";
import { useAuth } from "../auth/authContext";

const Navbar = () => {
  const auth = useAuth();

  return (
    <nav>
      <ul>
        <li className="primary">
          <NavLink to="/">Home</NavLink>
        </li>
        <li className="primary">
          <NavLink to="recipes">Recipes</NavLink>
        </li>
        {auth.user ? (
          <>
            <li className="secondary">
              <NavLink to="users">Profile</NavLink>
            </li>
            <li className="secondary">
              <NavLink to="users/signout">Sign-out</NavLink>
            </li>
          </>
        ) : (
          <>
            <li className="secondary">
              <NavLink to="users/signin">Sign-in</NavLink>
            </li>
            <li className="secondary">
              <NavLink to="users/signup">Sign-up</NavLink>
            </li>
          </>
        )}
      </ul>
    </nav>
  );
};

export default Navbar;
