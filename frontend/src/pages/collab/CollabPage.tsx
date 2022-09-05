import { Flex } from "@chakra-ui/react";
import { useContext } from "react";
import { useQuery } from "react-query";
import CollabBlock from "../../components/collab/CollabBlock";
import { AuthContext } from "../../providers/AuthProvider";
import { Collab, getCollabsByName } from "../../services/SourceService";

export default function CollabPage(): JSX.Element {

  const { user } = useContext(AuthContext)
  const { isLoading, data: collabs } = useQuery(["collabs", user], () => getCollabsByName(user?.email),
  {
    enabled: !!user?.email
  });  
 
  return (
    <>
        <Flex direction={"column"}>
          {!isLoading &&
           collabs !== null && 
           collabs != undefined &&
           collabs.map((collab: Collab) => (
            <CollabBlock key={collab.id} collab={collab} userReacts={true}/>
           ))}
        </Flex>
    </>
  );
}