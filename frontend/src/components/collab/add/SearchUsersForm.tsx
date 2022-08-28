import { SearchIcon } from "@chakra-ui/icons";
import { Flex } from "@chakra-ui/layout";
import { IconButton, Input, InputGroup, InputRightElement } from "@chakra-ui/react";
import { useState } from "react";

interface SearchProps {
    callback: (keyword: string) => void
}

export default function SearchUsersForm({ callback }: SearchProps): JSX.Element {

  const [search, setSearch] = useState<string>("");

  return (
    <>
        <Flex 
            alignItems={"center"}
            alignContent="space-between" 
            justifyContent={"space-between"}
            padding={"10px"}
        >
            <Flex>Search:</Flex>
            <Flex>
                <InputGroup>
                    <Input
                    value={search}
                    onChange={(e) => setSearch(e.target.value)}
                    placeholder='Enter user email'
                    />
                    <InputRightElement
                        children={
                            <IconButton
                                aria-label={"Search Users"}
                                variant={"ghost"}
                                icon={<SearchIcon/>}
                                onClick={() => callback(search)}
                            />
                        }
                    />
                </InputGroup>
            </Flex>
        </Flex>
    </>
  );
}