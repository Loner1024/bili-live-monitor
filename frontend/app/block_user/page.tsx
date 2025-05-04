'use client'
import React, {useState} from 'react';
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "@/components/table";
import {QueryClient, QueryClientProvider, useQuery, useQueryClient} from "@tanstack/react-query";
import {Loading} from "@/components/loading";
import {getFormatTime} from "@/utils/utils";
import {Pagination, PaginationList, PaginationNext, PaginationPage, PaginationPrevious} from "@/components/pagination";
import MySidebar from "@/components/func/sidebar";
import Link from "next/link";
import {useStreamer} from "@/context/StreamersContext";


interface QueryResponseData {
    code: number
    message: string
    count: number
    data: BlockUserData[]
}

interface BlockUserData {
    uid: number,
    username: string,
    operator: number,
    room_id: number,
    timestamp: number,
    block_expired: number,
}

const queryClient = new QueryClient();

const DataTable = () => {
    const queryClient = useQueryClient();
    const [limit] = useState(15);
    const [offset, setOffset] = useState(0);
    const {streamers: streamerData} = useStreamer()

    const {data: response, isLoading} = useQuery<QueryResponseData, Error>(
        {
            queryKey: [`data`, limit, offset],
            queryFn: () => fetcher({
                limit, offset
            }),
            refetchOnWindowFocus: false,
        },
        queryClient,
    );

    const handleNextPage = () => {
        setOffset(prevOffset => prevOffset + limit);
    };

    const handlePrevPage = () => {
        setOffset(prevOffset => Math.max(prevOffset - limit, 0));
    };

    const jumpToPage = (i: number) => {
        setOffset(i * limit);
    }

    const totalPage = Math.ceil(response == null ? 0 : response?.count / limit);
    const currentPage = (offset / limit) + 1;
    const startPage = currentPage > 3 ? currentPage - 3 : 0
    const endPage = Math.min(currentPage > 3 ? currentPage + 1 : 4, totalPage - 1)
    const range = (start: number, stop: number, step: number) =>
        Array.from({length: (stop - start) / step + 1}, (_, i) => start + i * step);

    return (
        <MySidebar room_id={""}>
            <div className={"flex flex-col"}>
                <div className="max-w-2xl mx-auto w-full h-full bg-yellow-100 rounded-lg p-4 shadow-md">
                    <div className="flex items-start space-x-3">
                        <svg className="w-5 h-5 flex-shrink-0 text-yellow-800" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd"
                                  d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
                                  clip-rule="evenodd"/>
                        </svg>
                        <div className="flex-1">
                            <h3 className="text-sm font-semibold text-yellow-800">警告</h3>
                            <p className="mt-1 text-sm text-yellow-700">由于 B 站不再给普通用户推送房间的封禁信息，四害榜已经失去数据源，该功能已失效！</p>
                        </div>
                    </div>
                </div>

                <div className={"flex flex-col mt-8"}>
                    {isLoading ? <Loading/> : null}
                    <Table hidden={isLoading}>
                        <TableHead>
                            <TableRow>
                                <TableHeader>uid</TableHeader>
                                <TableHeader>昵称</TableHeader>
                                <TableHeader>操作人</TableHeader>
                                <TableHeader>封禁直播间</TableHeader>
                                <TableHeader>封禁时间</TableHeader>
                                <TableHeader>预计解封时间</TableHeader>
                            </TableRow>
                        </TableHead>
                        <TableBody>
                            {response?.data?.map((data: BlockUserData, index: number) => {
                                return <TableRow key={index}>
                                    <TableCell>
                                        <Link className={"underline-offset-* text-cyan-700"}
                                              href={`/checker/${data.uid}`}>{data.uid}</Link>
                                    </TableCell>
                                    <TableCell className="font-medium">{data.username}</TableCell>
                                    <TableCell>{data.operator == 1 ? "房管" : (data.operator == 2 ? "主播" : "其他")}</TableCell>
                                    <TableCell>{streamerData.find(x => x.room_id == data.room_id)?.nickname}</TableCell>
                                    <TableCell>{getFormatTime(data.timestamp)}</TableCell>
                                    <TableCell>{getFormatTime(data.block_expired)}</TableCell>
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
            </div>
        </MySidebar>
    );
}

const CheckerPage = () => {
    return (
        <QueryClientProvider client={queryClient}>
            <DataTable/>
        </QueryClientProvider>
    )

};

const fetcher = async (params: {
    limit: number,
    offset: number,
}): Promise<QueryResponseData> => {
    const query = new URLSearchParams({
        limit: params.limit.toString(),
        offset: params.offset.toString(),
    });

    const response = await fetch(`${process.env.API_URL}/api/block_user?${query}`);

    if (!response.ok) {
        throw new Error('Network response was not ok');
    }

    return response.json();
};


export default CheckerPage;