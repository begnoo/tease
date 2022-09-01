import { Box, Flex } from "@chakra-ui/layout";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faFolder } from '@fortawesome/free-solid-svg-icons';
import { faFile } from '@fortawesome/free-solid-svg-icons';
import { Item } from "../../../services/StorageService";

interface SourceBrowserProp {
    items: Item[],
    push: any
}

export default function SourceBrowser({ items, push }: SourceBrowserProp): JSX.Element {

  return (
    <Box borderWidth={"2px"} padding={"10px 10px 10px 10px"}>
      {items.map(item => (
          <Flex 
            _hover={
              { cursor: 'pointer', color: 'black', backgroundColor: 'gray.400' }
            }
            key={item.sha1 + item.name} 
            alignContent={"space-between"} 
            justifyContent={"space-between"}
            onClick={() => push(item)}
            padding={"10px"}
            width={"100%"}
            >
              <Flex>
                <FontAwesomeIcon icon={item.dtype == "blob" ? faFile : faFolder} />
              </Flex>
              <Flex>
                {item.name}
              </Flex>
          </Flex>
      ))}
    </Box>
  );
}