import { Flex } from "@chakra-ui/layout";
import { Source } from "../../../services/SourceService";
import SourceBlock from "./SourceBlock";

interface SourceListProps {
  sources: Source[],
  owner?: boolean,
}

export default function SourceList({ sources, owner }: SourceListProps): JSX.Element {

    return (
      <Flex flexDirection="column" width={"100%"} gap={"10px"} mb={"50px"}>
          {sources.map((source) => (
              <SourceBlock key={source.id} source={source} owner={owner}/>
          ))}
      </Flex>
    );
}