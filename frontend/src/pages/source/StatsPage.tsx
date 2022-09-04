import { Flex, Tab, TabList, TabPanel, TabPanels, Tabs } from "@chakra-ui/react";
import { useQuery } from "react-query";
import { useParams } from "react-router";
import CollabDoughnut from "../../components/source/stats/CollabDoughnut";
import CommitsLineChart from "../../components/source/stats/CommitsLineChart";
import { readCommitsStatsByDate, readCommitsStatsByUser } from "../../services/StatsService";

export default function StatsPage(): JSX.Element {

  const { user, source } = useParams();
  const { isLoading: loadingCommitStatsByDay, data: commitStatsByDay } = useQuery(["commitStatsByDay", user, source], () => readCommitsStatsByDate({user, source}),
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
            <Tab key={2}>Added & Deleted</Tab>
          </TabList>

        <TabPanels>
          <TabPanel key={1}>
            {!loadingCommitStatsByUser 
              && commitStatsByUser !== undefined
              && commitStatsByUser.length !== 0 
              && <CollabDoughnut items={commitStatsByUser}/>}
          </TabPanel>
          <TabPanel key={2}>
          {!loadingCommitStatsByDay 
            && commitStatsByDay !== undefined 
            && <CommitsLineChart items={commitStatsByDay}/>}
          </TabPanel>
        </TabPanels>
      </Tabs>


    </>
  );
}