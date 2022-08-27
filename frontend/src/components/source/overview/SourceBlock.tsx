import { Button } from "@chakra-ui/button";
import { Box, Center, Flex } from "@chakra-ui/layout";
import { Link, useNavigate } from "react-router-dom";
import { Source } from "../../../services/SourceService";
import { howMuchAgo } from "../../../utils/dateUtils"

interface SourceBoxProp {
  source: Source
}

export default function SourceBlock({ source }: SourceBoxProp): JSX.Element {

  const navigate = useNavigate();

  return (
    <Box mt={2} borderWidth={"2px"} padding={"30px 30px 20px 30px"}>
      <Flex alignContent={"space-between"} justifyContent={"space-between"}>
        <Flex>{source.name}</Flex>
        <Flex>
          created { howMuchAgo(source.createdAt) }
        </Flex>
      </Flex>
      <Flex alignItems={"center"} justifyContent={"space-between"}>
        <Flex gap={"10px"}> 
          <Flex>author:</Flex>
          <Flex>
            <a href="#" onClick={() => navigate(`/source/${source.owner}`)} >{source.owner}</a>
          </Flex>
        </Flex>
        <Flex>
          <Button mt={"5px"} size={"sm"} onClick={() => navigate(`/source/${source.owner}/${source.name}`)}>
            View
          </Button>
        </Flex>
      </Flex>
    </Box>
  );
}