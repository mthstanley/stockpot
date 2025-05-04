import { NavLink } from "react-router";
import { useAuth } from "../auth/authContext";

const Navbar = () => {
  const auth = useAuth();

  return (
    <nav>
      <div className="content">
        <h2 className="brand">Stockpot</h2>
        <ul>
          <li>
            <NavLink to="/">Home</NavLink>
          </li>
          <li>
            <NavLink to="recipes">Recipes</NavLink>
          </li>
          {auth.user ? (
            <>
              <li>
                <NavLink to="users">Profile</NavLink>
              </li>
              <li>
                <NavLink to="users/signout">Sign-out</NavLink>
              </li>
            </>
          ) : (
            <>
              <li>
                <NavLink to="users/signin">Sign-in</NavLink>
              </li>
              <li>
                <NavLink to="users/signup">Sign-up</NavLink>
              </li>
            </>
          )}
        </ul>
      </div>
    </nav>
  );
};

export default Navbar;
