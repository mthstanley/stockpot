import {ReactNode, useState} from "react";
import {apiClient, GetTokenResponse} from "../../utils/api";
import {Navigate, useLocation} from "react-router";
import {AuthContext, AuthUser, useAuth} from "./authContext";


const AuthProvider = ({children}: {children: ReactNode}) => {
    const [user, setUser] = useState<AuthUser | null>(null);

    const signin = (username: string, password: string, callback: VoidFunction) => {
        const tokenResponse: GetTokenResponse = apiClient.login(username, password);
        setUser({username, token: tokenResponse.token});
        callback();
    };

    const signout = (callback: VoidFunction) => {
        apiClient.logout();
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
