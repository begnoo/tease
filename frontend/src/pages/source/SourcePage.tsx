import { Flex } from "@chakra-ui/layout";
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, IconButton, Input, InputGroup, InputRightElement, Portal, Select } from "@chakra-ui/react";
import { useEffect, useState } from "react";
import { useQuery } from "react-query";
import { useParams } from "react-router";
import BlobView from "../../components/source/overview/BlobView";
import SourceBrowser from "../../components/source/overview/SourceBrowser";
import { BranchContent, Item, readBlob, readBranches, readTree } from "../../services/StorageService";
import { AtSignIcon, CopyIcon, InfoIcon } from '@chakra-ui/icons'
import { SOURCE_CLONE_URL } from "../../constatns";
import { useNavigate } from "react-router-dom";
import { fromMilis } from "../../utils/dateUtils";

interface ItemWithId extends Item{
  id: number
}

export default function SourcePage(): JSX.Element {

  const [ trailStack, setTrailStack ] = useState<ItemWithId[]>([]);
  const [ tree, setTree ] = useState<string>();
  const [ blob, setBlob ] = useState<string>();
  const { user, source } = useParams();
  const [ selectedBranch, setSelectedBranch ] = useState<BranchContent>();
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
    const masterBranch = branches?.find((branch) => branch.name === "master");
    if (masterBranch == undefined) {
      return;
    }
    setSelectedBranch(masterBranch);
  }, [JSON.stringify(branches)]);

  useEffect(() => {
    setBlob(undefined);
    setTree(selectedBranch?.tree_sha1);
    !!selectedBranch && setTrailStack([{sha1: selectedBranch?.tree_sha1, name: "root", dtype: "tree", id: 0}])
  }, [selectedBranch]);

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

  const navigate = useNavigate();

  return (
    <>

    {!branchIsLoading && branches !== undefined && branches !== null && 
      <Select defaultValue={"master"} onChange={(e) => {
        console.log(e.target.value);
        const newBranch = branches?.find((branch) => branch.name === e.target.value);
        console.log(newBranch);
        if (newBranch == undefined) {
          return;
        }
        setSelectedBranch(newBranch);
      }}>
        {branches.map(branch => (
          <option key={branch.tree_sha1 + branch.name} value={branch.name}>{branch.name}</option>
        ))}
      </Select>
    }      
    {!!selectedBranch && <Flex
      _hover={
        { cursor: 'pointer', color: 'black', backgroundColor: 'gray.400' }
      }
      borderRadius={"10px"}
      onClick={() => navigate(`/source/${user}/${source}/commits/${selectedBranch.name}`)}
      mt="5px"
      width={"100%"}
      direction={"column"}
      borderWidth={"2px"}
      color={"gray.400"} 
      fontSize={"14px"} 
      padding={"15px"}
    >
        <Flex
          alignContent="space-between" 
          justifyContent={"space-between"}
        >
          <Flex direction={"column"}>
              {selectedBranch.commit.author}
          </Flex>
          <Flex>{fromMilis(selectedBranch.commit.date)}</Flex>
        </Flex>
        <Flex>
          {selectedBranch.commit.sha1}
        </Flex>
        <Flex>
          <Flex>
            Message: {selectedBranch.commit.message}
          </Flex>
        </Flex>
    </Flex>}
    <Flex
      borderRadius={"10px"}
      mt="5px"
      mb={"5px"}
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
        />
        <InputRightElement
          borderColor={"gray"}
          children={
            <IconButton
              colorScheme={"white"}
              variant={"ghost"}
              aria-label='Copy to clipboard'
              size={"sm"}
              icon={<CopyIcon/>}
              onClick={copyToClipboard}
            />} 
        />
    </InputGroup>
    </Flex>
    
    <Flex
      mb={"5px"}
      borderRadius={"10px"}
      padding={"10px"}
      borderWidth={"2px"}>
      <Breadcrumb>
        {trailStack.map(itemWithId => (
          <BreadcrumbItem key={itemWithId.id}>
            <BreadcrumbLink onClick={() => back(itemWithId)}>{itemWithId.name}</BreadcrumbLink>
          </BreadcrumbItem>
        ))}
      </Breadcrumb>
    </Flex>
    
    <Flex
      mb={"5px"}
      flexDirection="column">
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
    <Flex
      borderRadius={"10px"}
      borderWidth={"2px"}
      color={"gray.400"} 
      fontSize={"14px"} 
      alignItems={"center"}
      alignContent="space-between" 
      justifyContent={"space-between"}
      padding={"10px"}
    >
      <Flex>Collabs</Flex>
      <Flex>
        <IconButton
          onClick={() => navigate(`/source/${user}/${source}/collabs`)}
          aria-label="Collabs"
          variant={"ghost"}
          children={
            <AtSignIcon/>
          }
        />
      </Flex>
    </Flex>

    <Flex
      mt={"5px"}
      borderRadius={"10px"}
      borderWidth={"2px"}
      color={"gray.400"} 
      fontSize={"14px"} 
      alignItems={"center"}
      alignContent="space-between" 
      justifyContent={"space-between"}
      padding={"10px"}
    >
      <Flex>Stats</Flex>
      <Flex>
        <IconButton
          onClick={() => navigate(`/source/${user}/${source}/stats`)}
          aria-label="Stats"
          variant={"ghost"}
          children={
            <InfoIcon/>
          }
        />
      </Flex>
    </Flex>
    </>
  );
}