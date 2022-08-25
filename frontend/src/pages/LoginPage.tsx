import { Center, Flex } from "@chakra-ui/layout";
import { useContext } from "react";
import LoginForm from "../components/users/login/LoginForm";
import { AuthContext } from "../providers/AuthProvider";

export default function LoginPage(): JSX.Element {
    // const { user } = useContext(AuthContext);

    return (
        <>
          <LoginForm />
        </>
    );
  }