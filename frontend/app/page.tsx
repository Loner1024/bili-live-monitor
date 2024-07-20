'use client'
import {Avatar} from '@/components/avatar'
import {Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu,} from '@/components/dropdown'
import {Navbar, NavbarSection, NavbarSpacer} from '@/components/navbar'
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from '@/components/table'
import {
    Sidebar,
    SidebarBody,
    SidebarFooter,
    SidebarHeader,
    SidebarItem,
    SidebarLabel,
    SidebarSection
} from '@/components/sidebar'
import {SidebarLayout} from '@/components/sidebar-layout'
import {Heading, Subheading} from "@/components/heading"
import {TextLink} from "@/components/text"
import {Input, InputGroup} from '@/components/input'
import {Select} from '@/components/select'
import {Button} from "@/components/button";
import {Pagination, PaginationList, PaginationNext, PaginationPage, PaginationPrevious} from '@/components/pagination'
import {FontAwesomeIcon} from '@fortawesome/react-fontawesome'
import {faBilibili} from "@fortawesome/free-brands-svg-icons";
import React, {useEffect, useState} from "react";
import {streamers} from "@/data/streamers";
import {faBackward} from "@fortawesome/free-solid-svg-icons";
import {format, fromUnixTime, parse} from "date-fns";
import {useTheme} from '@/context/ThemeContext';
import {ArrowPathIcon, ChevronDownIcon, MagnifyingGlassIcon, MoonIcon, SunIcon} from "@heroicons/react/24/solid";
import {Stat} from "@/components/stat";
import {QueryClient, QueryClientProvider, useQuery, useQueryClient} from '@tanstack/react-query';

interface streamer {
    id: number,
    nickname: string,
    username: string,
    bilibili_link: string,
    room_id: number,
    avatar: string,
    small_avatar: string,
    description: string,
}

export interface DanmuMessageResponse {
    code: number
    message: string
    count: number
    data: DanmuMessage[],
}

export interface DanmuMessage {
    uid: number
    username: string
    message: string
    message_type: string
    timestamp: number
    worth: number | undefined
}

interface QueryParam {
    timestamp: number
    message: string
    message_type: string
}

const queryClient = new QueryClient();


export default function Home() {
    const {theme, toggleTheme} = useTheme();
    const streamerData: streamer[] = streamers;
    const [selectedId, setSelectedId] = useState<number>(0);
    const room_id = streamerData[selectedId].room_id;


    const handleClick = (id: number) => {
        setSelectedId(id);
    };

    return (
        <div>
            <SidebarLayout
                navbar={
                    <Navbar>
                        <NavbarSpacer/>
                        <NavbarSection>
                        </NavbarSection>
                    </Navbar>
                }
                sidebar={
                    <Sidebar>
                        <SidebarHeader>
                            <div className={"flex flex-col gap-1 mt-9"}>
                                <Avatar
                                    src={streamerData[selectedId].avatar}/>
                                <Heading>{streamerData[selectedId].nickname}</Heading>
                                <TextLink
                                    href={streamerData[selectedId].bilibili_link}
                                    target={"_blank"}
                                    className={"no-underline hover:underline"}
                                >@{streamerData[selectedId].username}</TextLink>
                                <Subheading>{streamerData[selectedId].description}</Subheading>
                            </div>
                        </SidebarHeader>
                        <SidebarBody className={"justify-between"}>
                            <Dropdown>
                                <DropdownButton as={SidebarItem} className="lg:mb-2.5">
                                    <SidebarLabel className={"flex grow gap-2 items-center text-base font-medium"}>
                                        <FontAwesomeIcon icon={faBilibili} size={"2xs"} className={"size-8"}/>
                                        总监的同事们
                                    </SidebarLabel>
                                    <ChevronDownIcon/>
                                </DropdownButton>
                                <DropdownMenu className="min-w-80 lg:min-w-64" anchor="bottom start">
                                    <DropdownItem onClick={() => handleClick(1)}>
                                        <Avatar
                                            src="https://i1.hdslb.com/bfs/face/bcdce44b8cbe699292165bb3dd274046f9352702.jpg@240w_240h_1c_1s_!web-avatar-space-header.avif"
                                            className={"min-w-12 min-h-12"}
                                            square/>
                                        <DropdownLabel className={"text-lg"}>七七</DropdownLabel>
                                    </DropdownItem>
                                    {/*<DropdownDivider/>*/}
                                    <DropdownItem onClick={() => handleClick(2)}>
                                        <Avatar
                                            src="https://i0.hdslb.com/bfs/face/81e91eba2eeefff79f4507fe80b00ac3eb8d1f16.jpg@240w_240h_1c_1s_!web-avatar-space-header.avif"
                                            className={"min-w-12 min-h-12"}
                                            square/>
                                        <DropdownLabel className={"text-lg"}>小啾</DropdownLabel>
                                    </DropdownItem>
                                </DropdownMenu>
                            </Dropdown>
                            <div>
                                {selectedId != 0 ? <SidebarSection>
                                    <SidebarItem onClick={() => handleClick(0)} className={"text-base font-medium"}>
                                        <FontAwesomeIcon icon={faBackward} className={"size-6"}/>
                                        <SidebarLabel className={"ml-4"}>回去看卢</SidebarLabel>
                                    </SidebarItem>
                                </SidebarSection> : null}
                            </div>
                        </SidebarBody>
                        <SidebarFooter>
                            <div className={"flex cursor-pointer size-5"}
                                 onClick={toggleTheme}>
                                {theme == "dark" ? <MoonIcon color={"white"}/> : <SunIcon color={"black"}/>}
                            </div>
                        </SidebarFooter>
                    </Sidebar>
                }
            >
                {/* The page content */}

                <QueryClientProvider client={queryClient}>
                    <DataTable roomId={room_id.toString()}/>
                </QueryClientProvider>

            </SidebarLayout>
        </div>
    );
}

const baseURL = "https://zongjian.uniix.dev"
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

    const response = await fetch(`${baseURL}/api/${room_id}?${query}`);

    if (!response.ok) {
        throw new Error('Network response was not ok');
    }

    return response.json();
};

interface DataFetcherProps {
    roomId: string;
}

const DataTable: React.FC<DataFetcherProps> = ({roomId}) => {
    const [message, setMessage] = useState('');
    const [messageType, setMessageType] = useState('');
    const [timestamp, setTimestamp] = useState(getTimestampSecs(curDate));
    const [limit, setLimit] = useState(50);
    const [offset, setOffset] = useState(0);


    const queryClient = useQueryClient();

    const [queryParam, setQueryParam] = useState<QueryParam>({
        timestamp: getTimestampSecs(curDate),
        message: "",
        message_type: ""
    })

    useEffect(() => {
        setOffset(0)
        setMessageType("danmu")
        setMessage("")
        setTimestamp(getTimestampSecs(curDate))
        setQueryParam(prev => ({...prev, message: "", message_type: "danmu", timestamp: getTimestampSecs(curDate)}))
    }, [roomId]);

    const {data: danmuData, error, isLoading} = useQuery<DanmuMessageResponse, Error>(
        {
            queryKey: [`data`, roomId, queryParam, limit, offset],
            queryFn: () => fetcher({
                room_id: roomId,
                message: queryParam.message,
                message_type: queryParam.message_type,
                timestamp: queryParam.timestamp,
                limit,
                offset
            })
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
        <div>
            <div>
                <Heading>Today（统计数据是演示，暂时还没做完）</Heading>
                <div className="mt-4 grid gap-8 sm:grid-cols-2 xl:grid-cols-4 dark:text-white">
                    <Stat title="弹幕总数" value="$2.6M" change="+4.5%"/>
                    <Stat title="弹幕人数" value="$455" change="-0.5%"/>
                    <Stat title="SC总数" value="5,888" change="+4.5%"/>
                    <Stat title="SC总价值" value="823,067" change="+21.2%"/>
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
                        {isLoading ? <Loading />: null}
                        <Table hidden={isLoading}>
                            <TableHead>
                                <TableRow>
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
                </div>
                )
            }

const Loading = () => {
    return (
        <div className={"flex justify-center items-center h-96 w-full"}>
        <div className={"flex justify-center items-center size-1/12"}>
            <ArrowPathIcon className={"animate-spin text-gray-500"}/>
        </div>
    </div>
)
}

function getFormatTime(timestamp: number) {
    return format(fromUnixTime(timestamp), 'yyyy-MM-dd HH:mm:ss');
}

function getTimestampSecs(date: { getTime: () => number }) {
    return parseInt(String(date.getTime() / 1000))
}