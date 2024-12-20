import {ReactNode, useState} from "react";
import {apiClient, GetTokenResponse} from "../../utils/api";
import {Navigate, useLocation} from "react-router";
import {AuthContext, AuthUser, useAuth} from "./authContext";

const USER_STORAGE_KEY = "user";

const AuthProvider = ({children}: {children: ReactNode}) => {
    const userJson = localStorage.getItem(USER_STORAGE_KEY)
    const [user, setUser] = useState<AuthUser | null>(userJson && JSON.parse(userJson));

    const signin = (username: string, password: string, callback: VoidFunction) => {
        const tokenResponse: GetTokenResponse = apiClient.login(username, password);
        const newUser = {username, token: tokenResponse.token};
        localStorage.setItem(USER_STORAGE_KEY, JSON.stringify(newUser));
        setUser(newUser);
        callback();
    };

    const signout = (callback: VoidFunction) => {
        apiClient.logout();
        localStorage.removeItem(USER_STORAGE_KEY);
        setUser(null);
        callback();
    };

    const value = {user, signin, signout};

    return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};

export const RequireAuth = ({ children }: { children: JSX.Element }) => {
  const auth = useAuth();
  const location = useLocation();

  if (!auth.user) {
    // Redirect them to the /signin page, but save the current location they were
    // trying to go to when they were redirected. This allows us to send them
    // along to that page after they login, which is a nicer user experience
    // than dropping them off on the home page.
    return <Navigate to="/users/signin" state={{ from: location }} replace />;
  }

  return children;
};


export default AuthProvider;
