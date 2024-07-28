'use client'
import React, {useState} from 'react';
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "@/components/table";
import {QueryClient, QueryClientProvider, useQuery, useQueryClient} from "@tanstack/react-query";
import {Loading} from "@/components/loading";
import {getFormatTime} from "@/utils/utils";
import {streamers} from "@/data/streamers"
import {Pagination, PaginationList, PaginationNext, PaginationPage, PaginationPrevious} from "@/components/pagination";


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
}

const queryClient = new QueryClient();
const streamerData = streamers

const DataTable = () => {
    const queryClient = useQueryClient();
    const [limit] = useState(15);
    const [offset, setOffset] = useState(0);

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
        <div className={"flex flex-col"}>
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
                        </TableRow>
                    </TableHead>
                    <TableBody>
                        {response?.data?.map((data: BlockUserData, index: number) => {
                            return <TableRow key={index}>
                                <TableCell>{data.uid}</TableCell>
                                <TableCell className="font-medium">{data.username}</TableCell>
                                <TableCell>{data.operator == 1 ? "房管" : (data.operator == 2 ? "主播": "其他")}</TableCell>
                                <TableCell>{streamerData.find(x => x.room_id == data.room_id)?.nickname}</TableCell>
                                <TableCell>{getFormatTime(data.timestamp)}</TableCell>
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