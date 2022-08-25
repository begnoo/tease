import { Center, Flex } from "@chakra-ui/layout";
import { useContext } from "react";
import { AuthContext } from "../providers/AuthProvider";

export default function HomePage(): JSX.Element {
    const { user } = useContext(AuthContext);

    return (
      <Flex mt={10} flexDirection="column">
        <Center>
          HOme home home
        </Center>
      </Flex>
    );
  }