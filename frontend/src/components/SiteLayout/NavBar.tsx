import { Button, Flex } from "@chakra-ui/react";
import { useContext } from "react";
import { Link, useNavigate } from "react-router-dom";
import { AUTH_ROUTES, CRED_ROUTES, ROUTES } from "../../constatns";
import { AuthContext } from "../../providers/AuthProvider";

export default function NavBar() {

    const navigate = useNavigate();
    const { user, logout } = useContext(AuthContext);

    return (
        <Flex padding={"15px"} alignContent="space-between" justifyContent={"space-between"}>    
            <Flex>
                {ROUTES.map((route, rid) => (
                    <Link key={rid} to={route.href}>
                        <Button variant="ghost" aria-label="Home" my={5} w="100%">
                            {route.name}
                        </Button>
                    </Link>
                ))}
                {user !== null && AUTH_ROUTES.map((route, rid) => (
                    <Link key={rid} to={route.href}>
                        <Button variant="ghost" aria-label="Home" my={5} w="100%">
                            {route.name}
                        </Button>
                    </Link>
                ))}
            </Flex>
            {user === null && 
            <Flex>
                {CRED_ROUTES.map((route, rid) => (
                    <Link key={rid} to={route.href}>
                        <Button variant="ghost" aria-label="Home" my={5} w="100%">
                            {route.name}
                        </Button>
                    </Link>
                ))}
            </Flex>}
            {user !== null && 
            <Flex>
                {<a>
                    <Button 
                        variant="ghost"
                        my={5} w="100%"
                        onClick={() => {
                            logout()
                            setTimeout(() => navigate("/"), 500);
                        }}>
                        LOGOUT
                    </Button>
                </a>}
            </Flex>}
        </Flex>
    );

}