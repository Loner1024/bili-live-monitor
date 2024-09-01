'use client'
import React, {useState} from 'react';
import {Input, InputGroup} from "@/components/input";
import {ArrowPathIcon, MagnifyingGlassIcon} from "@heroicons/react/24/solid";
import {Button} from "@/components/button";
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "@/components/table";
import {QueryClient, QueryClientProvider, useQuery, useQueryClient} from "@tanstack/react-query";
import {Loading} from "@/components/loading";
import {parse} from "date-fns";
import {getFormatTime, getTimestampSecs} from "@/utils/utils";
import {streamers} from "@/data/streamers"
import MySidebar from "@/components/func/sidebar";
import {useRouter, useSearchParams} from "next/navigation";


interface QueryParam {
    uid: number,
    timestamp: number,
}

interface QueryResponseData {
    code: number
    message: string
    data: CheckerData[]
}

interface CheckerData {
    uid: number,
    username: string,
    message: string,
    message_type: string,
    room_id: number,
    timestamp: number,
    worth: number,
}

const queryClient = new QueryClient();
const streamerData = streamers
const curDate = new Date();

const DataTable = ({params}: {params:{uid: number}}) => {
    const router = useRouter();
    const queryClient = useQueryClient();
    const [uid, setUid] = useState(params.uid);
    const [timestamp, setTimestamp] = useState(getTimestampSecs(curDate));


    const {data: response, isLoading} = useQuery<QueryResponseData, Error>(
        {
            queryKey: [`data`, timestamp],
            queryFn: () => fetcher({
                uid: uid,
                timestamp: timestamp,
            }),
            refetchOnWindowFocus: false,
        },
        queryClient,
    );

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        router.push(`/checker/${uid}`);
    };

    return (
        <MySidebar room_id={""}>
            <div className={"flex flex-col"}>
                <div className={"mb-10"}>
                    <form onSubmit={handleSubmit} className={"flex flex-col justify-center mt-8 gap-4 md:flex-row"}>
                        <InputGroup>
                            <Input
                                onChange={(date) => setTimestamp(getTimestampSecs(parse(date.target.value, "yyyy-MM-dd", new Date())))}
                                name="date" type={"date"} defaultValue={getFormatTime(timestamp).substring(0, 10)}/>
                        </InputGroup>
                        <div className={"basis-1/3 mb-8 lg:mb-auto"}>
                            <InputGroup>
                                <MagnifyingGlassIcon/>
                                <Input
                                    onChange={(e) => setUid(parseInt(e.target.value))}
                                    name="search"
                                    placeholder="输入 uid 查询" aria-label="Search"/>
                            </InputGroup>
                        </div>
                        <Button
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
                                <TableHeader>弹幕类型</TableHeader>
                                <TableHeader>发送直播间</TableHeader>
                                <TableHeader>发送时间</TableHeader>
                                <TableHeader>价值</TableHeader>
                            </TableRow>
                        </TableHead>
                        <TableBody>
                            {response?.data?.map((data: CheckerData, index: number) => {
                                return <TableRow key={index}>
                                    <TableCell>{data.uid}</TableCell>
                                    <TableCell className="font-medium">{data.username}</TableCell>
                                    <TableCell>{data.message}</TableCell>
                                    <TableCell
                                        className="text-zinc-500">{data.message_type == "super_chat" ? "SC" : "弹幕"}</TableCell>
                                    <TableCell>{streamerData.find(x => x.room_id == data.room_id)?.nickname}</TableCell>
                                    <TableCell>{getFormatTime(data.timestamp)}</TableCell>
                                    <TableCell>{data.worth != undefined ? data.worth : 0.0}</TableCell>
                                </TableRow>
                            })}
                        </TableBody>
                    </Table>
                </div>
            </div>
        </MySidebar>
    );
}

const CheckerPage = ({params}: {params: {uid: number}}) => {
    return (
        <QueryClientProvider client={queryClient}>
            <DataTable params={params}/>
        </QueryClientProvider>
    )

};

const fetcher = async (params: { timestamp: number; uid: number }): Promise<QueryResponseData> => {
    const query = new URLSearchParams({
        uid: params.uid.toString(),
        timestamp: params.timestamp.toString(),
    });

    const response = await fetch(`${process.env.API_URL}/api/checker?${query}`);

    if (!response.ok) {
        throw new Error('Network response was not ok');
    }

    return response.json();
};


export default CheckerPage;