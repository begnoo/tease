import { CheckIcon, CloseIcon, DeleteIcon } from "@chakra-ui/icons";
import { Flex, IconButton } from "@chakra-ui/react";
import { AxiosError } from "axios";
import { useMutation, useQueryClient } from "react-query";
import { useRequestToast } from "../../hooks/useRequestToast";
import { acceptCollab, Collab, deleteCollab, rejectCollab } from "../../services/SourceService";
import { timeDistance } from "../../utils/dateUtils";

interface CollabBlockProps {
    collab: Collab,
    userReacts?: boolean,
    userCreated?: boolean
}

export default function CollabBlock({ collab, userReacts, userCreated }: CollabBlockProps): JSX.Element {

  const queryClient = useQueryClient();
  const { toastSuccess, toastFailure } = useRequestToast("You've successfully deleted a collab.", "Couldn't delete collab")
  const { mutate: delCollab } = useMutation(
      deleteCollab,
      {
          onSuccess: (_res) => {
              queryClient.invalidateQueries("collabs");
              toastSuccess();
          },
          onError: (err: AxiosError) => {
              toastFailure(err);
          }
      }
  );
  
  const { mutate: rejCollab } = useMutation(
    rejectCollab,
    {
        onSuccess: (_res) => {
            queryClient.invalidateQueries("collabs");
            toastSuccess();
        },
        onError: (err: AxiosError) => {
            toastFailure(err);
        }
    }
  );

  const { mutate: accCollab } = useMutation(
    acceptCollab,
    {
        onSuccess: (_res) => {
            queryClient.invalidateQueries("collabs");
            toastSuccess();
        },
        onError: (err: AxiosError) => {
            toastFailure(err);
        }
    }
  );
 
  return (
    <>
        <Flex
            borderRadius={"10px"}
            mt={"10px"}
            flexDirection={"column"}
            borderWidth={"2px"}
            color={"gray.400"} 
            alignContent="space-between" 
            justifyContent={"space-between"}
            padding={"20px"}>
                <Flex>{userReacts ? "From: " + collab.from : "To: " + collab.name}</Flex>
                <Flex>Source: {collab.sourceName}</Flex>
                <Flex>expiers in: { timeDistance(collab.expiersAt) }</Flex>
                <Flex>
                    {!collab.reactedToInvite && <Flex></Flex>}
                </Flex>
                <Flex 
                    alignItems={"center"}
                    alignContent={"space-between"} 
                    justifyContent={"space-between"}>
                    <Flex gap={"10px"}>
                        <Flex>State:</Flex>
                        {collab.reactedToInvite && collab.acceptedInvite && <Flex>Accepted</Flex>}
                        {collab.reactedToInvite && !collab.acceptedInvite && <Flex>Refused</Flex>}
                        {!collab.reactedToInvite && !collab.acceptedInvite && <Flex>Pending</Flex>}
                    </Flex>
                    {userCreated && <Flex>
                        <IconButton
                            aria-label="Delete Collab" 
                            variant={"ghost"}
                            icon={
                                <DeleteIcon/>
                            }
                            onClick={() => delCollab(collab.id)}/>
                    </Flex>}
                </Flex>
                {userReacts && !collab.reactedToInvite && <Flex 
                    alignItems={"center"}
                    gap="10px"
                    flexDirection={"row-reverse"}
                    >
                    <Flex>
                        <IconButton
                            aria-label="Reject Collab" 
                            variant={"outline"}
                            icon={
                                <CloseIcon/>
                            }
                            onClick={() => rejCollab(collab.id)}/>
                    </Flex>
                    <Flex>
                        <IconButton
                            aria-label="Accept Collab" 
                            variant={"outline"}
                            icon={
                                <CheckIcon/>
                            }
                            onClick={() => accCollab(collab.id)}/>
                    </Flex>    
                </Flex>}
        </Flex>
    </>
  );
}
