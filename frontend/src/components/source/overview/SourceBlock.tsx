import { Button } from "@chakra-ui/button";
import { Box, Center, Flex } from "@chakra-ui/layout";
import { AxiosError } from "axios";
import { useMutation, useQueryClient } from "react-query";
import { Link, useNavigate } from "react-router-dom";
import { useRequestToast } from "../../../hooks/useRequestToast";
import { deleteSource, Source } from "../../../services/SourceService";
import { howMuchAgo } from "../../../utils/dateUtils"

interface SourceBoxProp {
  source: Source
  owner?: boolean
}

export default function SourceBlock({ source, owner }: SourceBoxProp): JSX.Element {

  const navigate = useNavigate();
  const queryClient = useQueryClient();
  const { toastSuccess, toastFailure } = useRequestToast("You've successfully deleted a source.", "Couldn't delete source")
  const { mutate: delSource } = useMutation(
      deleteSource,
      {
          onSuccess: (_res) => {
              queryClient.invalidateQueries("user_sources");
              queryClient.invalidateQueries("all_sources");
              toastSuccess();
          },
          onError: (err: AxiosError) => {
              toastFailure(err);
          }
      }
  );

  return (
    <Box 
      borderRadius={"10px"} 
      borderWidth={"2px"} 
      padding={"30px 30px 20px 30px"}>
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
          {owner && <Button ml={"5px"} mt={"5px"} size={"sm"} onClick={() => delSource(source.id)}>
            Delete
          </Button>}
        </Flex>
      </Flex>
    </Box>
  );
}