'use client'
import React, {useEffect, useState} from "react";
import {getFormatTime, getTimestampSecs, StatisticsResult} from "@/utils/utils";
import {QueryClient, QueryClientProvider, useQuery} from "@tanstack/react-query";
import {Heading} from "@/components/heading";
import {Stat} from "@/components/stat";
import {Input, InputGroup} from "@/components/input";
import {parse} from "date-fns";
import {ArrowPathIcon, MagnifyingGlassIcon} from "@heroicons/react/24/solid";
import {Select} from "@/components/select";
import {Button} from "@/components/button";
import {Loading} from "@/components/loading";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "@/components/table";
import {Pagination, PaginationList, PaginationNext, PaginationPage, PaginationPrevious} from "@/components/pagination";
import {DanmuChart} from "@/components/chart/danmu_chart";
import Link from "next/link";

const statisticsFetcher = async (params: {
    room_id: string,
    timestamp: number,
}): Promise<StatisticsResponse> => {
    const query = new URLSearchParams({
        timestamp: params.timestamp.toString(),
        room_id: params.room_id
    });

    const response = await fetch(`${process.env.API_URL}/api/statistics?${query}`);

    if (!response.ok) {
        throw new Error('Network response was not ok');
    }

    return response.json();
};


interface QueryParam {
    timestamp: number,
    message: string,
    message_type: string
}

interface StatisticsChange {
    danmu_total_change: number,
    danmu_people_change: number,
    super_chat_total_change: number,
    super_chat_worth_change: number,
}

const curDate = new Date();

const fetcher = async (params: {
    room_id: string,
    message?: string,
    message_type?: string,
    timestamp: number,
    limit: number,
    offset: number
}): Promise<DanmuMessageResponse> => {
    const {room_id, ...queryParameters} = params;
    const query = new URLSearchParams({
        timestamp: queryParameters.timestamp.toString(),
        limit: queryParameters.limit.toString(),
        offset: queryParameters.offset.toString(),
    });
    queryParameters.message != "" ? query.set("message", queryParameters.message || '') : null;
    queryParameters.message_type != "" ? query.set("message_type", queryParameters.message_type || 'danmu') : null;

    const response = await fetch(`${process.env.API_URL}/api/${room_id}?${query}`);

    if (!response.ok) {
        throw new Error('Network response was not ok');
    }

    return response.json();
};

function get_change_str(change: number): string {
    if (change >= 0) {
        return `+${change.toFixed(2)}%`;
    } else {
        return `${change.toFixed(2)}%`;
    }
}

export const DataTable = (params: { room_id: string }) => {
    const [message, setMessage] = useState('');
    const [messageType, setMessageType] = useState('');
    const [timestamp, setTimestamp] = useState(getTimestampSecs(curDate));
    const [limit] = useState(50);
    const [offset, setOffset] = useState(0);
    const room_id = params.room_id;
    const queryClient = new QueryClient();


    const [queryParam, setQueryParam] = useState<QueryParam>({
        timestamp,
        message: "",
        message_type: "danmu"
    })

    let {data: statisticsData, error, isLoading: statisticsIsLoading} = useQuery<StatisticsResponse, Error>(
        {
            queryKey: [`statistics`, room_id],
            queryFn: () => statisticsFetcher({
                room_id: room_id,
                timestamp: queryParam.timestamp,
            }),
        },
        queryClient
    );
    statisticsData = statisticsData?.code == -1 ? new StatisticsResponse() : statisticsData;

    let today = statisticsData?.data.today || new StatisticsResult()
    let yesterday = statisticsData?.data.yesterday || new StatisticsResult()
    const statisticsChange = calcChange(today, yesterday);

    useEffect(() => {
        setOffset(0)
        setMessageType("danmu")
        setMessage("")
        setTimestamp(getTimestampSecs(curDate))
        setQueryParam({message: "", message_type: "danmu", timestamp: getTimestampSecs(curDate)})
    }, [room_id]);

    const {data: danmuData, isLoading} = useQuery<DanmuMessageResponse, Error>(
        {
            queryKey: [`data`, room_id, queryParam, limit, offset],
            queryFn: () => fetcher({
                room_id: room_id,
                message: queryParam.message,
                message_type: queryParam.message_type,
                timestamp: queryParam.timestamp,
                limit,
                offset
            }),
        },
        queryClient
    );

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        setOffset(0)
        setQueryParam(prev => ({...prev, message: message, message_type: messageType, timestamp: timestamp}));
    };

    const handleNextPage = () => {
        setOffset(prevOffset => prevOffset + limit);
    };

    const handlePrevPage = () => {
        setOffset(prevOffset => Math.max(prevOffset - limit, 0));
    };

    const jumpToPage = (i: number) => {
        setOffset(i * limit);
    }

    const totalPage = Math.ceil(danmuData == null ? 0 : danmuData?.count / limit);
    const currentPage = (offset / limit) + 1;
    const startPage = currentPage > 3 ? currentPage - 3 : 0
    const endPage = Math.min(currentPage > 3 ? currentPage + 1 : 4, totalPage - 1)
    const range = (start: number, stop: number, step: number) =>
        Array.from({length: (stop - start) / step + 1}, (_, i) => start + i * step);

    return (
        <QueryClientProvider client={queryClient}>
            <div className={"mb-10"}>
                <DanmuChart room_id={params.room_id}/>
            </div>
            <div>
                <Heading>Today</Heading>
                <div className="mt-4 grid gap-8 sm:grid-cols-2 xl:grid-cols-4 dark:text-white">
                    <Stat title="弹幕总数" value={today?.danmu_total == undefined ? "" : today.danmu_total.toString()}
                          change={get_change_str(statisticsChange.danmu_total_change)}/>
                    <Stat title="弹幕人数" value={today?.danmu_people == undefined ? "" : today.danmu_people.toString()}
                          change={get_change_str(statisticsChange.danmu_people_change)}/>
                    <Stat title="SC总数"
                          value={today?.super_chat_total == undefined ? "" : today.super_chat_total.toString()}
                          change={get_change_str(statisticsChange.super_chat_total_change)}/>
                    <Stat title="SC总价值"
                          value={today?.super_chat_worth == undefined ? "" : today.super_chat_worth.toString()}
                          change={get_change_str(statisticsChange.super_chat_worth_change)}/>
                </div>
            </div>
            <div className={"mb-8"}>
                <form onSubmit={handleSubmit} className={"flex flex-col mt-8 gap-4 md:flex-row"}>
                    <InputGroup>
                        <Input
                            onChange={(date) => setTimestamp(getTimestampSecs(parse(date.target.value, "yyyy-MM-dd", new Date())))}
                            name="date" type={"date"} defaultValue={getFormatTime(timestamp).substring(0, 10)}/>
                    </InputGroup>
                    <div className={"basis-1/2"}>
                        <InputGroup>
                            <MagnifyingGlassIcon/>
                            <Input value={message}
                                   onChange={(data) => setMessage(data.target.value)}
                                   name="search"
                                   placeholder="Search&hellip;" aria-label="Search"/>
                        </InputGroup>
                    </div>
                    <div className={"basis-1/4"}>
                        <Select
                            value={messageType}
                            onChange={(data) => setMessageType(data.target.value)}
                            aria-label="message type"
                            name="message_type">
                            <option value="danmu">弹幕</option>
                            <option value="super_chat">SC</option>
                        </Select>
                    </div>
                    <Button
                        disabled={isLoading}
                        type={"submit"}
                        className={"basis-1/4"}
                        color={"dark/zinc"}>
                        {isLoading ? <ArrowPathIcon className={"animate-spin"}/> : null}
                        {!isLoading ? <MagnifyingGlassIcon/> : null}
                        搜索
                    </Button>
                </form>
            </div>
            <div className={"flex flex-col mt-8"}>
                {isLoading ? <Loading/> : null}
                <Table hidden={isLoading}>
                    <TableHead>
                        <TableRow>
                            <TableHeader>uid</TableHeader>
                            <TableHeader>昵称</TableHeader>
                            <TableHeader>弹幕内容</TableHeader>
                            <TableHeader>类型</TableHeader>
                            <TableHeader>发送时间</TableHeader>
                            <TableHeader>价值</TableHeader>
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {danmuData?.data?.map((danmu: DanmuMessage, index: number) => {
                            return <TableRow key={index}>
                                <TableCell>
                                    <Link className={"underline-offset-* text-cyan-700"} href={`/checker/${danmu.uid}`}>{danmu.uid}</Link>
                                </TableCell>
                                <TableCell className="font-medium">{danmu.username}</TableCell>
                                <TableCell>{danmu.message}</TableCell>
                                <TableCell
                                    className="text-zinc-500">{danmu.message_type == "super_chat" ? "SC" : "弹幕"}</TableCell>
                                <TableCell>{getFormatTime(danmu.timestamp)}</TableCell>
                                <TableCell>{danmu.worth != undefined ? danmu.worth : 0.0}</TableCell>
                            </TableRow>
                        })}
                    </TableBody>
                </Table>

                <Pagination className={"mt-8"}>
                    <PaginationPrevious disable={currentPage == 1} onClick={handlePrevPage}/>
                    <PaginationList>
                        {
                            range(startPage, endPage, 1).map((i) => (
                                <PaginationPage className={"hover:cursor-pointer"} onClick={() => jumpToPage(i)}
                                                key={i}
                                                current={i + 1 === currentPage}>
                                    {i + 1}
                                </PaginationPage>
                            ))
                        }
                    </PaginationList>
                    <PaginationNext disable={currentPage == totalPage} onClick={handleNextPage}/>
                </Pagination>
            </div>
        </QueryClientProvider>
    )
}

function calcChange(today: StatisticsResult, yesterday: StatisticsResult): StatisticsChange {
    const danmu_total_change = (today.danmu_total - yesterday.danmu_total) / yesterday.danmu_total * 100;
    const danmu_people_change = (today.danmu_people - yesterday.danmu_people) / yesterday.danmu_people * 100;
    const super_chat_total_change = (today.super_chat_total - yesterday.super_chat_total) / yesterday.super_chat_total * 100;
    const super_chat_worth_change = (today.super_chat_worth - yesterday.super_chat_worth) / yesterday.super_chat_worth * 100;
    return {
        danmu_total_change,
        danmu_people_change,
        super_chat_total_change,
        super_chat_worth_change
    }
}



export class StatisticsData {
    today: StatisticsResult = new StatisticsResult()
    yesterday: StatisticsResult = new StatisticsResult()
}

export class StatisticsResponse {
    code: number = 0
    message: string = "success"
    data: StatisticsData = new StatisticsData()
}


export interface DanmuMessageResponse {
    code: number
    message: string
    count: number
    data: DanmuMessage[]
}

export interface DanmuMessage {
    uid: number
    username: string
    message: string
    message_type: string
    timestamp: number
    worth: number | undefined
}
