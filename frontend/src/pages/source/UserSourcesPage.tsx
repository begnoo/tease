import { Center, Flex } from "@chakra-ui/layout";
import { useQuery } from "react-query";
import { useParams } from "react-router";
import SourceList from "../../components/source/overview/SourceList";
import { readSourcesByUser } from "../../services/SourceService";

export default function UserSourcesPage(): JSX.Element {

  const { user } = useParams();
  const { isLoading, data: sources } = useQuery(["user_sources", user], () => readSourcesByUser(user),
  {
    enabled: !!user
  });  

  return (
    <Flex mt={10} flexDirection="column">
      <Center>
        {!isLoading && sources !== null && sources !== undefined && <SourceList sources={sources}/>}
      </Center>
    </Flex>
  );
}