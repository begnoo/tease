import React, { useState } from "react";
import { useQueryClient } from "react-query";
import { addUserIntoLocalStorage, getUserFromLocalStorage, parseJwt } from "../utils/jwtUtils";

interface User {
    email: string,
    role: string
}

type ContextProps = {
  user: User | null;
  login: any;
  logout: any;
};
// Provider for user auth and data
export const AuthContext = React.createContext<Partial<ContextProps>>({});

export const AuthProvider = ({ children }: any) => {
    
    const [user, setUser] = useState<User | null>(null);

    const login = () => {
        const jwtObject = parseJwt(getUserFromLocalStorage().token);
        const role = jwtObject.role;
        const email = jwtObject.email;

        setUser({
            email,
            role
        });
    };

    const logout = () => {
        setUser(null);
        addUserIntoLocalStorage({});
    };

    return (
        <AuthContext.Provider value={{
            user,
            login,
            logout 
        }}>
            {children}
        </AuthContext.Provider>
    );
};