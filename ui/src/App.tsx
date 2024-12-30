import "./App.css";
import { Route, Routes } from "react-router";
import HomePage from "./components/Home";
import SignupPage from "./components/user/Signup";
import AuthProvider, { RequireAuth } from "./components/auth/AuthProvider";
import ProfilePage from "./components/user/Profile";
import SigninPage from "./components/user/Signin";
import RecipesPage from "./components/recipe/RecipesPage";
import RecipeDetailPage from "./components/recipe/RecipeDetailPage";

function App() {
  return (
    <AuthProvider>
      <Routes>
        <Route index element={<HomePage />} />
        <Route path="users">
          <Route
            index
            element={
              <RequireAuth>
                <ProfilePage />
              </RequireAuth>
            }
          />
          <Route path="signup" element={<SignupPage />} />
          <Route path="signin" element={<SigninPage />} />
        </Route>
        <Route path="recipes">
          <Route index element={<RecipesPage />} />
          <Route path=":id" element={<RecipeDetailPage />} />
        </Route>
      </Routes>
    </AuthProvider>
  );
}

export default App;
