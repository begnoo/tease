import { AddIcon, SearchIcon } from "@chakra-ui/icons";
import { Flex } from "@chakra-ui/layout";
import { IconButton, Input, InputGroup, InputRightElement, useDisclosure } from "@chakra-ui/react";
import { AddCollabModal } from "./AddCollabModal";

interface CollabViewProps {
    source: string | undefined,
    user: string | undefined
}

export default function CollabView({ source, user }: CollabViewProps): JSX.Element {

  const { isOpen, onOpen, onClose } = useDisclosure();
    
  return (
    <>
        <Flex 
            alignContent="space-between" 
            justifyContent={"space-between"}
        >
            <Flex>Add</Flex>
            <Flex>
                <IconButton
                    onClick={() => onOpen()}
                    aria-label="Add Collab"
                    variant={"outline"}
                    children={
                        <AddIcon/>
                    }
                />
            </Flex>
        </Flex>
        <AddCollabModal isOpen={isOpen} onClose={onClose} source={source} owner={user}/>
    </>
  );
}