import { Box, Container, Flex } from "@chakra-ui/react";
import NavBar from "./NavBar";

export default function SiteLayout({ children }: {children: any}) {

    return (
        <>
            <NavBar />
            <Container maxWidth="100vh">
                {children}
            </Container>
        </>
    );

}