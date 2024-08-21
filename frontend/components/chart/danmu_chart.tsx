"use client"

import * as React from "react"
import {CartesianGrid, Line, LineChart, XAxis} from "recharts"

import {Card, CardContent, CardDescription, CardHeader, CardTitle,} from "@/components/ui/card"
import {ChartConfig, ChartContainer, ChartTooltip, ChartTooltipContent,} from "@/components/ui/chart"
import {format} from "date-fns";
import {QueryClient, useQuery} from "@tanstack/react-query";
import {StatisticsResult} from "@/utils/utils";

const chartConfig = {
    views: {
        label: "数量",
    },
    danmu_total: {
        label: "弹幕数量",
        color: "hsl(var(--chart-1))",
    },
    danmu_people: {
        label: "弹幕人数",
        color: "hsl(var(--chart-2))",
    },
    super_chat_total: {
        label: "SC数量",
        color: "hsl(var(--chart-3))",
    },
    super_chat_worth: {
        label: "SC价值",
        color: "hsl(var(--chart-4))",
    },
} satisfies ChartConfig

const queryClient = new QueryClient();

const danmuStatisticsFetcher = async (params: {room_id: string, start: number, end: number}): Promise<DanmuStatisticsResponse> => {
    const query = new URLSearchParams({
        room_id: params.room_id,
        start: params.start.toFixed().toString(),
        end: params.end.toFixed().toString(),
    });

    const response = await fetch(`${process.env.API_URL}/api/danmu_statistics?${query}`);

    if (!response.ok) {
        throw new Error('Network response was not ok');
    }

    return response.json();
};

export class DanmuStatisticsResponse {
    code: number = 0
    message: string = "success"
    data: StatisticsResult[] = []
}


export function DanmuChart(params: {room_id: string}) {
    const [activeChart, setActiveChart] =
        React.useState<keyof typeof chartConfig>("danmu_total")
    const room_id = params.room_id
    const end = Date.now() / 1000
    const start = end - 30*24*60*60

    let {data: statisticsData, error, isLoading: statisticsIsLoading} = useQuery<DanmuStatisticsResponse, Error>(
        {
            queryKey: [`danmuStatistics`],
            queryFn: () => danmuStatisticsFetcher({
                room_id,
                start,
                end
            }),
        },
        queryClient
    );

    const chartData = statisticsData?.data || [];

    const formattedChartData = chartData.map((data) => ({
        ...data,
        date: format(new Date(data.timestamp * 1000), "yyyy/MM/dd"),
    }))

    const total = React.useMemo(
        () => ({
            danmu_total: chartData.reduce((acc, curr) => acc + curr.danmu_total, 0),
            danmu_people: chartData.reduce((acc, curr) => acc + curr.danmu_people, 0),
            super_chat_total: chartData.reduce((acc, curr) => acc + curr.super_chat_total, 0),
            super_chat_worth: chartData.reduce((acc, curr) => acc + curr.super_chat_worth, 0),
        }),
        []
    )

    return (
        <Card>
            <CardHeader className="flex flex-col items-stretch space-y-0 border-b p-0 sm:flex-row">
                <div className="flex flex-1 flex-col justify-center gap-1 px-6 py-5 sm:py-6">
                    <CardTitle>一些统计数据</CardTitle>
                    <CardDescription>
                        并不靠谱
                    </CardDescription>
                </div>
                <div className="flex">
                    {["danmu_total", "danmu_people", "super_chat_total", "super_chat_worth"].map((key) => {
                        const chart = key as keyof typeof chartConfig
                        return (
                            <button
                                key={chart}
                                data-active={activeChart === chart}
                                className="flex flex-1 flex-col justify-center gap-1 border-t px-6 py-4 text-left even:border-l data-[active=true]:bg-muted/50 sm:border-l sm:border-t-0 sm:px-8 sm:py-6"
                                onClick={() => setActiveChart(chart)}
                            >
                <span className="text-xs text-muted-foreground">
                  {chartConfig[chart].label}
                </span>
                                <span className="text-lg font-bold leading-none sm:text-3xl">
                  {total[key as keyof typeof total].toLocaleString()}
                </span>
                            </button>
                        )
                    })}
                </div>
            </CardHeader>
            <CardContent className="px-2 sm:p-6">
                <ChartContainer
                    config={chartConfig}
                    className="aspect-auto h-[250px] w-full"
                >
                    <LineChart
                        accessibilityLayer
                        data={formattedChartData}
                        margin={{
                            left: 12,
                            right: 12,
                        }}
                    >
                        <CartesianGrid vertical={false}/>
                        <XAxis
                            dataKey="date"
                            tickLine={false}
                            axisLine={false}
                            tickMargin={8}
                            minTickGap={32}
                            tickFormatter={(value) => {
                                const date = new Date(value)
                                return date.toLocaleDateString("zh-cn", {
                                    month: "short",
                                    day: "numeric",
                                })
                            }}
                        />
                        <ChartTooltip
                            content={
                                <ChartTooltipContent
                                    className="w-[150px]"
                                    nameKey="views"
                                    labelFormatter={(value) => {
                                        // value.toLocaleDateString("zh-cn")
                                        return new Date(value).toLocaleDateString("zh-cn")
                                    }}
                                />
                            }
                        />
                        <Line
                            dataKey={activeChart}
                            type="monotone"
                            stroke={`var(--color-${activeChart})`}
                            strokeWidth={2}
                            dot={false}
                        />
                    </LineChart>
                </ChartContainer>
            </CardContent>
        </Card>
    )
}


