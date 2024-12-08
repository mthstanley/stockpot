import {createContext, ReactNode, useContext, useState} from "react";
import {apiClient} from "../utils/api";
import {Navigate, useLocation} from "react-router";

interface AuthUser {
    username: string;
    token: string;
}

interface AuthContextType {
    user: AuthUser;
    signin: (username: string, password: string, callback: VoidFunction) => void;
    signout: (callback: VoidFunction) => void;
}

let AuthContext = createContext<AuthContextType>(null!);

const AuthProvider = ({children}: {children: ReactNode}) => {
    let [user, setUser] = useState<any>(null);

    let signin = (username: string, password: string, callback: VoidFunction) => {
        const token = apiClient.login(username, password);
        setUser({username, token});
        callback();
    };

    let signout = (callback: VoidFunction) => {
        apiClient.logout();
        setUser(null);
        callback();
    };

    let value = {user, signin, signout};

    return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};

export const RequireAuth = ({ children }: { children: JSX.Element }) => {
  let auth = useAuth();
  let location = useLocation();

  if (!auth.user) {
    // Redirect them to the /signin page, but save the current location they were
    // trying to go to when they were redirected. This allows us to send them
    // along to that page after they login, which is a nicer user experience
    // than dropping them off on the home page.
    return <Navigate to="/users/signin" state={{ from: location }} replace />;
  }

  return children;
};

export const useAuth = () => {
    return useContext(AuthContext);
};

export default AuthProvider;
