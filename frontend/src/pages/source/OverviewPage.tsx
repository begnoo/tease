import { Center, Flex } from "@chakra-ui/layout";
import { useQuery } from "react-query";
import SourceList from "../../components/source/overview/SourceList";
import { readSources } from "../../services/SourceService";

export default function OverviewPage(): JSX.Element {

  const {isLoading, data: sources} = useQuery("all_sources", () => readSources());  

  return (
    <Flex mt={10}>
      {!isLoading && sources !== null && sources !== undefined && <SourceList sources={sources}/>}
    </Flex>
  );
}