import { AddIcon } from "@chakra-ui/icons";
import { Box, Button, Flex, IconButton, Modal, ModalBody, ModalCloseButton, ModalContent, ModalFooter, ModalHeader, ModalOverlay } from "@chakra-ui/react"
import { AxiosError } from "axios";
import { useEffect, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "react-query";
import { useRequestToast } from "../../../hooks/useRequestToast";
import { addCollab } from "../../../services/SourceService";
import { searchUsers } from "../../../services/UserService";
import SearchUsersForm from "./SearchUsersForm"

interface AddCollabProps {
    isOpen: boolean,
    onClose: () => void
    source: string | undefined,
    owner: string | undefined,
}

export function AddCollabModal({isOpen, onClose, source, owner}: AddCollabProps) {

    const queryClient = useQueryClient();
    const [search, setSearch] = useState<string>("");
    const {isLoading, data: users, refetch} = useQuery(["users_search", search], () => searchUsers(search), {
        enabled: !!search 
    });
    const { toastSuccess, toastFailure } = useRequestToast("You've successfully added a collab.", "Couldn't add collab")
    const { mutate: postCollab } = useMutation(
        addCollab,
        {
            onSuccess: (res) => {
                queryClient.invalidateQueries("collabs");
                toastSuccess();
            },
            onError: (err: AxiosError) => {
                toastFailure(err);
            }
        }
    );

    const addCallback = (collabarator: string) => {
        const req = {
            owner,
            name: source,
            collabarator
        }
        postCollab(req);
    }

    useEffect(() => {
        if (search !== "") {
          refetch();
        }
    }, [search]);

    return (
      <>  
        <Modal isOpen={isOpen} onClose={onClose}>
          <ModalOverlay />
          <ModalContent>
            <ModalHeader>Add Collab</ModalHeader>
            <ModalCloseButton />
            
            <ModalBody>
                <SearchUsersForm callback={setSearch}/>
                <Flex direction={"column"}>
                    {!isLoading &&
                      users != null &&
                      users != undefined &&
                      !!users && users.map((obj: any, index: number) => (
                        <Flex
                        alignItems={"center"}
                        alignContent="space-between" 
                        justifyContent={"space-between"}
                        padding={"10px"}
                        key={index}
                        onClick={() => addCallback(obj.email)}
                        >
                          <Flex>{obj.email}</Flex>
                          <Flex>
                            <IconButton
                              aria-label="Add"
                              variant={"ghost"}
                              children={
                                <AddIcon/>
                              }
                            />
                          </Flex>
                        </Flex>
                    ))}
                </Flex>
            </ModalBody>
  
            <ModalFooter>
            </ModalFooter>
          </ModalContent>
        </Modal>
      </>
    )
  }