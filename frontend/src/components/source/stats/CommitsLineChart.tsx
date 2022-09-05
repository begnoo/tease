import { Box } from "@chakra-ui/layout";
import { Chart as ChartJS, Legend, Tooltip, TimeSeriesScale, LinearScale, BarElement, CategoryScale, TimeScale, PointElement, LineElement, Title, Filler } from "chart.js";
import { useEffect, useState } from "react";
import { Line } from "react-chartjs-2";
import { CommitStatsByDay } from "../../../services/StatsService";
import 'chartjs-adapter-moment';
import { parse } from "date-fns";
import { Select } from "@chakra-ui/react";

interface CommitsBarChartProp {
    items: CommitStatsByDay[]
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

export default function CommitsLineChart({ items }: CommitsBarChartProp): JSX.Element {

  const [data, setData] = useState<any>();
  const [unit, setUnit] = useState<any>("day");
  const [options, setOptions] = useState<any>();
  useEffect(() => {
    if (items.length == 0) {
        return;
    }

    setData(formatData(items));
  }, [JSON.stringify(items)]); 

  useEffect(() => {
    setOptions(getOptions(unit))
  }, [JSON.stringify(data), unit]);

  return (
    <>
    <Box
        backgroundColor={"whiteAlpha.800"}
        borderRadius={"10px"} 
        height={"400px"} 
        width={"700px"} 
        padding={"10px"}>
        {data !== undefined && <Line options={options} data={data} />}
    </Box>
    <Select mt={"10px"} onChange={(e) => setUnit(e.target.value)} value={unit}>
        <option value='day'>Per Day</option>
        <option value='week'>Per Week</option>
        <option value='month'>Per Month</option>
        <option value='year'>Per Year</option>
    </Select>
    </>
  );
}

interface TimePoint {
    x: Date,
    y: number
}

const formatData = (items: CommitStatsByDay[]): any => {

    const added_data: TimePoint[] = items.map((commit) => ({x: parse(commit.id, "dd-MM-yyyy", new Date()), y: commit.added}));
    const deleted_data: TimePoint[] = items.map((commit) => ({x: parse(commit.id, "dd-MM-yyyy", new Date()), y: commit.deleted}));
    added_data.sort((a, b) => a.x.getTime() > b.x.getTime() ? 1 : -1);
    deleted_data.sort((a, b) => a.x.getTime() > b.x.getTime() ? 1 : -1);

    return {
        datasets: [{
            fill: true,
            label: "added",
            data: added_data,
            borderColor: "green",
            backgroundColor: "rgba(0, 128, 0, 0.3)"
        },
        {
            fill: true,
            label: "deleted",
            data: deleted_data,
            borderColor: "red",
            backgroundColor: "rgba(255, 0, 0, 0.3)"
        }],
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
