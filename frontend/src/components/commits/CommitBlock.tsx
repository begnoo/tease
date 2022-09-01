import { Flex } from "@chakra-ui/react";
import { CommitItem } from "../../services/StorageService";
import { fromMilis, fromMilisTime } from "../../utils/dateUtils";

export const a = 3;

interface CommitBlockProps {
    commit: CommitItem
}

export default function CommitsBlock({ commit }: CommitBlockProps): JSX.Element {
  
    return (
      <>
        <Flex
          _hover={
            { cursor: 'pointer', color: 'black', backgroundColor: 'gray.400' }
          }
          onClick={() => console.log(commit.sha1)}
          mt="5px"
          width={"100%"}
          direction={"column"}
          borderWidth={"2px"}
          color={"gray.400"} 
          fontSize={"14px"} 
          padding={"15px"}
        >
            <Flex
              alignContent="space-between" 
              justifyContent={"space-between"}
            >
              <Flex direction={"column"}>
                  {commit.author}
              </Flex>
              <Flex direction={"column"}>
                <Flex>{fromMilis(commit.date)}</Flex>
              </Flex>
            </Flex>
            <Flex 
                alignContent="space-between" 
                justifyContent={"space-between"}
            >
              <Flex>{commit.sha1}</Flex>
              <Flex>{fromMilisTime(commit.date)}</Flex>
            </Flex>
            <Flex>
              <Flex>
                Message: {commit.message}
              </Flex>
            </Flex>
        </Flex>
      </>
    );
  }

