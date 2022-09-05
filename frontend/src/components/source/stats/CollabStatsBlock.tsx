import { Box, Flex } from "@chakra-ui/layout";
import { Chart as ChartJS, Legend, Tooltip, TimeSeriesScale, LinearScale, BarElement, CategoryScale, TimeScale, PointElement, LineElement, Title, Filler } from "chart.js";
import { useEffect, useState } from "react";
import { CommitDateAndCount, CommitStatsByCollabAndDate, CommitStatsByDay } from "../../../services/StatsService";
import 'chartjs-adapter-moment';
import { parse } from "date-fns";
import { Line } from "react-chartjs-2";

interface CollabStatsBlockProps {
    props: CommitStatsByCollabAndDate
}

ChartJS.register(
    TimeSeriesScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Filler,
    Legend
  );

export default function CollabStatsBlock({ props }: CollabStatsBlockProps): JSX.Element {

  const [data, setData] = useState<any>();
  useEffect(() => {
    if (props.items.length == 0) {
        return;
    }

    setData(formatData(props.items));
  }, [JSON.stringify(props.items)]); 

  return (
    <>
    <Flex
        alignItems={"center"}
        width={"330px"}
        direction={"column"} 
        padding={"5px"} 
        borderWidth={"3px"}>
        <Flex
            justifyContent={"space-between"}
            alignContent={"space-between"} 
            mt={"5px"} 
            ml={"5px"}>
            <Flex>{props.user}</Flex>
            <Flex color={"gray.500"} gap={"10px"}>
                (
                <Flex color={"green.400"}> + {props.added}</Flex>
                <Flex color={"red.300"}> - {props.deleted}</Flex>
                )
            </Flex>
        </Flex>
        <Flex
            mt={"5px"}
            mb={"5px"}
            backgroundColor={"whiteAlpha.800"}
            borderRadius={"10px"} 
            height={"150px"} 
            width={"300px"} 
            padding={"10px"}>
            {data !== undefined && <Line options={getOptions("day")} data={data} />}
        </Flex>
    </Flex>
    </>
  );
}

interface TimePoint {
    x: Date,
    y: number
}

// TODO: sredi grafik ovde tako da pokazuje count po danu
const formatData = (items: CommitDateAndCount[]): any => {

    const count_data: TimePoint[] = items.map((commit) => ({x: parse(commit.date, "dd-MM-yyyy", new Date()), y: commit.count}));
    count_data.sort((a, b) => a.x.getTime() > b.x.getTime() ? 1 : -1);
    
    return {
        datasets: [{
            fill: true,
            label: "commits",
            data: count_data,
            borderColor: "green",
            backgroundColor: "rgba(0, 128, 0, 0.3)"
        },],
        hoverOffset: 4
    }
}

const getOptions = (unit: string): any => {
    return {
        maintainAspectRatio: false,
        responsive: true,
        scales: {
            x: {
                type: 'timeseries',
                time: {
                    unit: unit,
                    round: unit
                }
            }   
        }
    }
}
