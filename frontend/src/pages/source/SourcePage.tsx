import { Flex } from "@chakra-ui/layout";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, IconButton, Input, InputGroup, InputRightElement, Select } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { useQuery } from "react-query";
import { useParams } from "react-router";
import BlobView from "../../components/source/overview/BlobView";
import SourceBrowser from "../../components/source/overview/SourceBrowser";
import { Item, readBlob, readBranches, readTree } from "../../services/StorageService";
import { CopyIcon } from '@chakra-ui/icons'
import { SOURCE_CLONE_URL } from "../../constatns";

interface ItemWithId extends Item{
  id: number
}

export default function SourcePage(): JSX.Element {

  const [ trailStack, setTrailStack ] = useState<ItemWithId[]>([]);
  const [ tree, setTree ] = useState<string>();
  const [ blob, setBlob ] = useState<string>();
  const { user, source } = useParams();
  const { isLoading: branchIsLoading, data: branches } = useQuery(
    ["branches", user, source], 
    () => readBranches({user, source}),
  {
    enabled: !!user && !!source
  });

  const { isLoading: treeIsLoading, data: items, refetch: getTreeItems } = useQuery(
    ["items", user, source, tree],
    () => readTree({user, source, sha: tree}),
  {
    enabled: false
  });

  const { isLoading: blobIsLoading, data: blobContent, refetch: getBlobContent } = useQuery(
    ["blob", user, source, tree],
    () => readBlob({user, source, sha: blob}),
  {
    enabled: false
  });  

  useEffect(() => {
    const master_tree = branches?.find((branch) => branch.name === "master")?.sha;
    if (master_tree !== undefined) {
      setTree(master_tree);
      setTrailStack([{sha1: master_tree, name: "root", dtype: "tree", id: 0}])
    }
  }, [JSON.stringify(branches)]);

  useEffect(() => {
    if (tree == undefined) {
      return;
    }
    getTreeItems();
  }, [tree]);
  

  useEffect(() => {
    if (blob == undefined) {
      return;
    }
    getBlobContent();
  }, [blob]);

  const push = (item: Item) => {
    const id = trailStack.length
    setTrailStack([...trailStack, {...item, id}]);
    if (item.dtype == "tree") {
      setTree(item.sha1);
    } else {
      setBlob(item.sha1)
    }
  }

  const back = (item: ItemWithId) => {
    if (item.dtype == "blob" || trailStack[trailStack.length - 1].id == item.id ) {
      return;
    }
    const new_trail = trailStack.slice(0, item.id+1);
    setTrailStack(new_trail);
    setTree(item.sha1);
    setBlob(undefined);
  }

  const copyToClipboard = () => {
    navigator.clipboard.writeText(`${SOURCE_CLONE_URL}/${user}/${source}`);
  }

  return (
    <>

    {!branchIsLoading && branches !== undefined && branches !== null && 
      <Select>
        {branches.map(branch => (
          <option key={branch.sha + branch.name} value={branch.sha}>{branch.name}</option>
        ))}
      </Select>
    }

    <Flex
      mt="5px"
      borderWidth={"2px"}
      alignContent="space-between" 
      justifyContent={"space-between"}
      alignItems={"center"}
      color={"gray.400"} 
      fontSize={"14px"} 
      padding={"10px"}
    >
      <Flex>Clone</Flex>
      <InputGroup width={"70%"}>
        <Input
          defaultValue={`${SOURCE_CLONE_URL}/${user}/${source}`}
          isReadOnly={true}
          placeholder='Enter password'
          // border={"0px"}
        />
        <InputRightElement
          borderColor={"gray"}
          children={
            <IconButton
              colorScheme={"teal"}
              variant={"ghost"}
              aria-label='Copy to clipboard'
              size={"sm"}
              icon={<CopyIcon/>}
              onClick={copyToClipboard}
            />} 
        />
    </InputGroup>
    </Flex>
    
    <Flex padding={2} borderWidth={"2px"}>
        <Breadcrumb>
          {trailStack.map(itemWithId => (
            <BreadcrumbItem key={itemWithId.id}>
              <BreadcrumbLink href='#' onClick={() => back(itemWithId)}>{itemWithId.name}</BreadcrumbLink>
            </BreadcrumbItem>
          ))}
        </Breadcrumb>
      </Flex>
    
      <Flex flexDirection="column">
          {!branchIsLoading &&
           !treeIsLoading &&
           blob === undefined &&
           items !== null &&
           items !== undefined &&
           <SourceBrowser items={items} push={push}/>}
          
          {!blobIsLoading &&
           blob != undefined &&
           blobContent != undefined &&
           blobContent != null &&
           <BlobView content={blobContent.content} size={blobContent.size}/>}
      </Flex>
    </>
  );
}