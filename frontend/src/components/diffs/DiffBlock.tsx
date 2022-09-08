import { Code, Flex, Text } from "@chakra-ui/react";
import { useMemo } from "react";
import { DiffItem } from "../../services/StorageService";

interface DiffBlockProps {
    diff: DiffItem
}

export default function DiffBlock({ diff }: DiffBlockProps): JSX.Element {

  return (
    <>
      <Flex
        justifyContent={"space-between"}
        alignContent={"space-between"} 
        key={diff.path}  
        mt={"5px"} 
        ml={"5px"}>
        <Flex key={diff.path}>{diff.path}</Flex>
        <Flex color={"gray.500"} gap={"10px"}>
          (
          <Flex color={"green.400"}> + {diff.added}</Flex>
          <Flex color={"red.300"}> - {diff.deleted}</Flex>
          )
        </Flex>
      </Flex>
      <Flex>
        <Flex
          borderRadius={"10px"}
          backgroundColor={"gray.700"}
          gap={"0px"}
          direction={"column"}
          width={"100%"}
          display={"block"}
          whiteSpace={"pre"}
          padding={"5px 10px 5px 10px"}>
          {diff.diff.map((line, index) => (
            <Text 
              key={index} 
              fontSize={"14px"} 
              padding={"0px"} 
              color={getColor(line as string)}
              style={{
                whiteSpace: "normal",
                wordWrap: "break-word"
              }}
              >
                {line}
              </Text>
          ))}
        </Flex>
      </Flex>
    </>
  );
}

const getColor = (line: string): string => {
  if (line.startsWith("+")) {
    return "green.400";
  } else if (line.startsWith("-")) {
    return "red.300";
  }
  return "white";
};
