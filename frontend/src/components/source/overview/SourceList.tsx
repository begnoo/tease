import { Flex } from "@chakra-ui/layout";
import { Source } from "../../../services/SourceService";
import SourceBlock from "./SourceBlock";

interface SourceListProps {
  sources: Source[],
}

export default function SourceList({ sources }: SourceListProps): JSX.Element {

    return (
      <Flex flexDirection="column" width={"100%"} gap={"10px"} mb={"50px"}>
          {sources.map((source) => (
              <SourceBlock key={source.id} source={source} />
          ))}
      </Flex>
    );
}