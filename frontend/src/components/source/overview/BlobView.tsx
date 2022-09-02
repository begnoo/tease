import { Box, Flex } from "@chakra-ui/layout";
import { Blob } from "../../../services/StorageService";
import CodeMirror from '@uiw/react-codemirror';
import { githubDark } from '@uiw/codemirror-theme-github';
import { javascript } from '@codemirror/lang-javascript';

export default function BlobView({ size, content }: Blob): JSX.Element {

  return (
    <Box 
      borderRadius={"10px"}
      borderWidth={"2px"} >
      <Flex 
        alignContent="space-between" 
        justifyContent={"space-between"}
        color={"gray.400"} 
        fontSize={"14px"} 
        padding={"10px 10px 8px 10px"}
        >
          <Flex>size ({size} bytes)</Flex>
          <Flex>Download</Flex>
        </Flex>
      <CodeMirror
      value={content}
      readOnly
      height="400px"
      theme={githubDark}
      extensions={[javascript({ typescript: true })]}
      />

    </Box>
  );
}