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
import {ChevronDownIcon,} from '@heroicons/react/16/solid'
import {MagnifyingGlassIcon} from '@heroicons/react/20/solid'

import {FontAwesomeIcon} from '@fortawesome/react-fontawesome'
import {faBilibili} from "@fortawesome/free-brands-svg-icons";
import {useState} from "react";
import {streamers} from "@/data/streamers";
import {faBackward} from "@fortawesome/free-solid-svg-icons";
import {format, fromUnixTime} from "date-fns";
import {Divider} from "@/components/divider";
import {Badge} from "@/components/badge";

interface streamer {
    id: number,
    nickname: string,
    username: string,
    bilibili_link: string,
    avatar: string,
    small_avatar: string,
    description: string,
}

export function Stat({title, value, change}: { title: string; value: string; change: string }) {
    return (
        <div>
            <Divider/>
            <div className="mt-6 text-lg/6 font-medium sm:text-sm/6">{title}</div>
            <div className="mt-3 text-3xl/8 font-semibold sm:text-2xl/8">{value}</div>
            <div className="mt-3 text-sm/6 sm:text-xs/6">
                <Badge color={change.startsWith('+') ? 'lime' : 'pink'}>{change}</Badge>{' '}
                <span className="text-zinc-500">from yesterday</span>
            </div>
        </div>
    )
}

export default function Home() {
    const streamerData: streamer[] = streamers;
    const [selectedId, setSelectedId] = useState<number>(0);
    const curDate = new Date();

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
                        <SidebarBody>
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
                        </SidebarBody>
                        <SidebarFooter>
                            {selectedId != 0 ? <SidebarSection>
                                <SidebarItem onClick={() => handleClick(0)} className={"text-base font-medium"}>
                                    <FontAwesomeIcon icon={faBackward} className={"size-6"}/>
                                    <SidebarLabel className={"ml-4"}>回去看卢</SidebarLabel>
                                </SidebarItem>
                            </SidebarSection> : null}
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
                    <Subheading className="mt-14">Recent orders</Subheading>
                </div>
                <div className={"mb-8"}>
                    <form className={"flex flex-col mt-8 gap-4 md:flex-row"}>
                        <InputGroup>
                            <Input name="date" type={"date"} defaultValue={curDate.toISOString().substring(0, 10)}/>
                        </InputGroup>
                        <div className={"basis-1/2"}>
                            <InputGroup>
                                <MagnifyingGlassIcon/>
                                <Input name="search" placeholder="Search&hellip;" aria-label="Search"/>
                            </InputGroup>
                        </div>
                        <div className={"basis-1/4"}>
                            <Select aria-label="message type" name="message_type">
                                <option value="danmu">弹幕</option>
                                <option value="super_chat">SC</option>
                            </Select>
                        </div>
                        <Button className={"basis-1/4"} color={"dark/zinc"}>搜索</Button>
                    </form>
                </div>
                <div className={"flex flex-col h-96 mt-8"}>
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
                            {users.map((user) => (
                                <TableRow key={user.uid}>
                                    <TableCell className="font-medium">{user.username}</TableCell>
                                    <TableCell>{user.message}</TableCell>
                                    <TableCell
                                        className="text-zinc-500">{user.message_type == "super_chat" ? "SC" : "弹幕"}</TableCell>
                                    <TableCell>{getFormatTime(user.timestamp)}</TableCell>
                                    <TableCell>{user.worth}</TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                    <Pagination className={"mt-8 self-end"}>
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

const users = [
    {
        uid: 1111,
        username: "Leslie Alexander",
        message_type: "danmu",
        message: "喝水吧",
        timestamp: 1720973747,
        worth: 30.0
    },
    {
        uid: 1111,
        username: "Leslie Alexander",
        message_type: "danmu",
        message: "喝水吧",
        timestamp: 1720973747,
        worth: 30.0
    },
    {
        uid: 1111,
        username: "Leslie Alexander",
        message_type: "danmu",
        message: "喝水吧",
        timestamp: 1720973747,
        worth: 30.0
    },
    {
        uid: 1111,
        username: "Leslie Alexander",
        message_type: "danmu",
        message: "喝水吧",
        timestamp: 1720973747,
        worth: 30.0
    },
    {
        uid: 1111,
        username: "Leslie Alexander",
        message_type: "danmu",
        message: "喝水吧",
        timestamp: 1720973747,
        worth: 30.0
    },
    {
        uid: 1111,
        username: "Leslie Alexander",
        message_type: "super_chat",
        message: "喝水吧",
        timestamp: 1720973747,
        worth: 30.0
    },
]