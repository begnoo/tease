import { Flex } from "@chakra-ui/react";
import { useQuery } from "react-query";
import { useParams } from "react-router";
import CollabView from "../../components/collab/add/CollabView";
import CollabBlock from "../../components/collab/CollabBlock";
import { getCollabs, Collab } from "../../services/SourceService";

export default function SourceCollabPage(): JSX.Element {

  const { user, source } = useParams();
  const { isLoading, data: collabs } = useQuery(["collabs", user, source], () => getCollabs(user, source),
  {
    enabled: !!user && !!source
  });  
 
  return (
    <>
        <CollabView user={user} source={source}/>
        <Flex direction={"column"}>
          {!isLoading &&
           collabs !== null && 
           collabs != undefined &&
           collabs.map((collab: Collab) => (
            <CollabBlock key={collab.id} collab={collab} userCreated={true}/>
           ))}
        </Flex>
    </>
  );
}