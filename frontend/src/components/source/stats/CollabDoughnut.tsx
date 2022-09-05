import { Box, Flex } from "@chakra-ui/layout";
import { Chart as ChartJS, ArcElement, Legend, Tooltip } from "chart.js";
import { useEffect, useState } from "react";
import { Doughnut } from "react-chartjs-2";
import { CommitStatsByCollab } from "../../../services/StatsService";

interface CollabDoughnutProps {
    items: CommitStatsByCollab[]
}

ChartJS.register(ArcElement, Tooltip, Legend);

export default function CollabDoughnut({ items }: CollabDoughnutProps): JSX.Element {

  const [data, setData] = useState<any>();
  const [percentage, setPercentage] = useState<number[]>([]);
  useEffect(() => {
    if (items.length == 0) {
        return;
    }
    
    let commits = items.map((collab) => collab.count);
    let totalCommits = commits.reduce((acc, add) => acc + add, 0);
    let percentages = commits.map((count: number) => Math.floor(count/totalCommits * 100));

    setPercentage(percentages);
    setData(formatData(items));
  }, [JSON.stringify(items)]); 

  return (
    <>
    <Box height={"400px"} width={"700px"} padding={"10px"}>
        {data !== undefined && <Doughnut data={data} options={getOptions(percentage)}/>}
    </Box>
    </>
  );
}

const formatData = (items: CommitStatsByCollab[]): any => {
    const labels = items.map((collab) => collab.user);
    const backgroundColor = items.map(() => random_rgba());
    return {
        labels,
        datasets: [{
            label: "ucinak",
            data: items.map((collab) => collab.count),
            backgroundColor,
            borderColor: backgroundColor
        }],
        hoverOffset: 4
    }
}

const getOptions = (percentages: number[]): any => {
    return {
        maintainAspectRatio: false,
        responsive: true,
        plugins: {
            legend: {
                position: 'left',
                align: 'left'
            },
            tooltip: {
                callbacks: {
                    label: (context: any) => {
                        var dataset = context.dataset.data[context.dataIndex];
                        var percentage = percentages[context.dataIndex];                  
                        return [context.label, `count: ${dataset}`, `percent: ${percentage}%`];
                    },
                },
            }
        }
    }
  }

const randomNum = () => Math.floor(Math.random() * (235 - 52 + 1) + 52);

function random_rgba() {
    var o = Math.round, r = Math.random, s = 255;
    return `rgb(${randomNum()}, ${randomNum()}, ${randomNum()})`;
}

