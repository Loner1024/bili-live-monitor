"use client"

import { TrendingUp } from "lucide-react"
import { CartesianGrid, Line, LineChart, XAxis } from "recharts"

import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@/components/ui/card"
import {
    ChartConfig,
    ChartContainer,
    ChartTooltip,
    ChartTooltipContent,
} from "@/components/ui/chart"
// const chartData = [
//     { month: "January", desktop: 186, mobile: 80 },
//     { month: "February", desktop: 305, mobile: 200 },
//     { month: "March", desktop: 237, mobile: 120 },
//     { month: "April", desktop: 73, mobile: 190 },
//     { month: "May", desktop: 209, mobile: 130 },
//     { month: "June", desktop: 214, mobile: 140 },
// ]

const chartConfig = {
    danmu_total: {
        label: "弹幕数量",
        color: "hsl(var(--chart-1))",
    },
} satisfies ChartConfig

export function Danmu_chart({chartData} : {chartData: any[]}) {
    return (
        <Card>
            <CardHeader>
                <CardTitle>弹幕数量</CardTitle>
                <CardDescription>7天弹幕数量</CardDescription>
            </CardHeader>
            <CardContent>
                <ChartContainer config={chartConfig}>
                    <LineChart
                        accessibilityLayer
                        data={chartData}
                        margin={{
                            left: 12,
                            right: 12,
                        }}
                    >
                        <CartesianGrid vertical={false} />
                        <XAxis
                            dataKey="timestamp"
                            tickLine={false}
                            axisLine={false}
                            tickMargin={8}
                            tickFormatter={(value) => new Date(value * 1000).toLocaleDateString()}
                        />
                        <ChartTooltip
                            cursor={false}
                            content={<ChartTooltipContent hideLabel />}
                        />
                        <Line
                            dataKey="danmu_total"
                            type="natural"
                            stroke="var(--color-danmu_total)"
                            strokeWidth={2}
                            dot={{
                                fill: "var(--color-danmu_total)",
                            }}
                            activeDot={{
                                r: 6,
                            }}
                        />
                    </LineChart>
                </ChartContainer>
            </CardContent>
            <CardFooter className="flex-col items-start gap-2 text-sm">
                {/*<div className="flex gap-2 font-medium leading-none">*/}
                {/*    Trending up by 5.2% this month <TrendingUp className="h-4 w-4" />*/}
                {/*</div>*/}
                {/*<div className="leading-none text-muted-foreground">*/}
                {/*    Showing total visitors for the last 6 months*/}
                {/*</div>*/}
            </CardFooter>
        </Card>
    )
}
