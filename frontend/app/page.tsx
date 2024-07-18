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
import React, {useState} from "react";
import {streamers} from "@/data/streamers";
import {faBackward} from "@fortawesome/free-solid-svg-icons";
import {format, fromUnixTime, parse} from "date-fns";
import {useTheme} from '@/context/ThemeContext';
import {ChevronDownIcon, MagnifyingGlassIcon, MoonIcon, SunIcon} from "@heroicons/react/24/solid";
import useSWR, {mutate} from 'swr'
import {Stat} from "@/components/stat";
import * as sea from "node:sea";

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

const fetcher = (url: string) => fetch(url).then((res) => res.json());
const curDate = new Date();
const baseURL = "http://43.138.202.44:31003"

export default function Home() {
    const {theme, toggleTheme} = useTheme();
    const streamerData: streamer[] = streamers;
    const [selectedId, setSelectedId] = useState<number>(0);
    const room_id = streamerData[selectedId].room_id;
    let timestamp = getTimestampSecs(curDate);
    const [searchText, setSearchText] = useState('');
    const [selectedOption, setSelectedOption] = useState('danmu');
    const [selectedDate, setSelectedDate] = useState<number>(getTimestampSecs(curDate));
    const [danmuData, setDanmuData] = useState<DanmuMessage[]>([]);


    const url = `${baseURL}/api/${room_id}`;

    const init_url = `${baseURL}/api/${room_id}?timestamp=${timestamp}&limit=50&offset=0`;
    const handleClick = (id: number) => {
        setSelectedId(id);
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        const query = new URLSearchParams({
            timestamp: String(selectedDate),
            limit: "50",
            offset: "0",
            message_type: selectedOption,
        });
        if (searchText != '') {
            query.set("message", searchText)
        }
        const newData: DanmuMessageResponse = await mutate(url + "?" + query.toString(), fetcher(url + "?" + query.toString()))
        setDanmuData(newData.data)
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
                <div>
                    <Heading>Today</Heading>
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
                                onChange={(date) => setSelectedDate(getTimestampSecs(parse(date.target.value, "yyyy-MM-dd", new Date())))}
                                name="date" type={"date"} defaultValue={curDate.toISOString().substring(0, 10)}/>
                        </InputGroup>
                        <div className={"basis-1/2"}>
                            <InputGroup>
                                <MagnifyingGlassIcon/>
                                <Input value={searchText} onChange={(e) => setSearchText(e.target.value)} name="search"
                                       placeholder="Search&hellip;" aria-label="Search"/>
                            </InputGroup>
                        </div>
                        <div className={"basis-1/4"}>
                            <Select onChange={(e) => setSelectedOption(e.target.value)} aria-label="message type"
                                    name="message_type">
                                <option value="danmu">弹幕</option>
                                <option value="super_chat">SC</option>
                            </Select>
                        </div>
                        <Button type={"submit"} className={"basis-1/4"} color={"dark/zinc"}>搜索</Button>
                    </form>
                </div>
                <div className={"flex flex-col mt-8"}>
                    <Table>
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
                            {danmuData?.map((danmu: DanmuMessage, index: number) => {
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
                        <PaginationPrevious href="?page=1"/>
                        <PaginationList>
                            <PaginationPage href="?page=1">1</PaginationPage>
                            <PaginationPage href="?page=2" current>
                                2
                            </PaginationPage>
                            <PaginationPage href="?page=3">3</PaginationPage>
                        </PaginationList>
                        <PaginationNext href="?page=3"/>
                    </Pagination>
                </div>
            </SidebarLayout>
        </div>

    );
}

function getFormatTime(timestamp: number) {
    return format(fromUnixTime(timestamp), 'yyyy-MM-dd HH:mm:ss');
}

function getTimestampSecs(date: { getTime: () => number }) {
    return parseInt(String(date.getTime() / 1000))
}