import { Flex } from "@chakra-ui/react";
import { useQuery } from "react-query";
import { useParams } from "react-router-dom";
import CommitsBlock from "../../components/commits/CommitBlock";
import { readCommits } from "../../services/StorageService";

export default function CommitsPage(): JSX.Element {

  const { user, source, branch } = useParams();
  const { isLoading: commitsAreLoading, data: commits } = useQuery(
    ["commits", user, source], 
    () => readCommits({user, source, branch}),
  {
    enabled: !!user && !!source && !!branch
  });

  return (
    <>
      {!commitsAreLoading && commits?.map((commit) => (
        <Flex key={commit.sha1}>
          <CommitsBlock commit={commit}/>
        </Flex>
      ))}
    </>
  );
}