import { Flex } from "@chakra-ui/react";
import { useNavigate } from "react-router-dom";
import { CommitItem } from "../../services/StorageService";
import { fromMilis, fromMilisTime } from "../../utils/dateUtils";

export const a = 3;

interface CommitBlockProps {
    user?: string,
    source?: string,
    showParent?: boolean,
    commit: CommitItem
}

export default function CommitsBlock({ user, source, commit, showParent }: CommitBlockProps): JSX.Element {
  
    const navigate = useNavigate();

    return (
      <>
        <Flex
          _hover={
            { cursor: 'pointer', color: 'black', backgroundColor: 'gray.400' }
          }
          onClick={() => !!user && !!source && navigate(`/source/${user}/${source}/commits/diff/${commit.sha1}`)}
          mt="5px"
          borderRadius={"10px"}
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
            {showParent && commit.parents.length !== 3 && <Flex direction={"column"}>
              Parents:
              <Flex>
                <Flex>
                {commit.parents[0]}
                </Flex>
                {commit.parents.length > 1 && <Flex>
                +{commit.parents[1]}
                </Flex>}
              </Flex>
            </Flex>}
            <Flex>
              <Flex>
                Message: {commit.message}
              </Flex>
            </Flex>
        </Flex>
      </>
    );
  }

