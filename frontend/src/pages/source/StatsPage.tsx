import { Flex, Tab, TabList, TabPanel, TabPanels, Tabs } from "@chakra-ui/react";
import { useQuery } from "react-query";
import { useParams } from "react-router";
import CollabDoughnut from "../../components/source/stats/CollabDoughnut";
import { readCommitsStats, readCommitsStatsByUser } from "../../services/StatsService";

export default function StatsPage(): JSX.Element {

  const { user, source } = useParams();
  const { isLoading: loadingCommitStats, data: commitStats } = useQuery(["commitStats", user, source], () => readCommitsStats({user, source}),
  {
    enabled: !!user && !!source
  });  

  const { isLoading: loadingCommitStatsByUser, data: commitStatsByUser } = useQuery(["commitStatsByUser", user, source], () => readCommitsStatsByUser({user, source}),
  {
    enabled: !!user && !!source
  });  
 
  return (
    <>
        <Tabs>
          <TabList>
            <Tab key={1}>Collabarators Effect</Tab>
            <Tab>Two</Tab>
          </TabList>

        <TabPanels>
          <TabPanel key={1}>
            {!loadingCommitStatsByUser 
              && commitStatsByUser !== undefined
              && commitStatsByUser.length !== 0 
              && <CollabDoughnut items={commitStatsByUser}/>}
          </TabPanel>
          <TabPanel key={"tab2"}>
          {!loadingCommitStats 
            && commitStats !== undefined 
            && commitStats.map((commit) => (
            <Flex key={commit.id}>{commit.sha}</Flex>
          ))}
          </TabPanel>
        </TabPanels>
      </Tabs>


    </>
  );
}