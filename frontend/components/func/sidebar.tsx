'use client'
import {SidebarLayout} from "@/components/sidebar-layout";
import {Navbar, NavbarSection, NavbarSpacer} from "@/components/navbar";
import {
    Sidebar,
    SidebarBody,
    SidebarFooter,
    SidebarHeader,
    SidebarItem,
    SidebarLabel,
    SidebarSection
} from "@/components/sidebar";
import {Avatar} from "@/components/avatar";
import {Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu} from "@/components/dropdown";
import {BackOne, Bug, Search, TvOne} from "@icon-park/react";
import {ChevronDownIcon, MoonIcon, SunIcon} from "@heroicons/react/24/solid";
import {streamers} from "@/data/streamers";
import React from "react";
import {useTheme} from "@/context/ThemeContext";
import {Heading, Subheading} from "@/components/heading";
import {TextLink} from "@/components/text";

export default function MySidebar({
                                      children, room_id
                                  }: Readonly<{
    children: React.ReactNode;
    room_id: string;
}>) {
    const {theme, toggleTheme} = useTheme();
    const room_info = streamers.find((streamer) => streamer.room_id.toString() == room_id)

    return (
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
                                src={room_info?.avatar}/>
                            <Heading>{room_info?.nickname}</Heading>
                            <TextLink
                                href={room_info?.bilibili_link || "#"}
                                target={"_blank"}
                                className={"no-underline hover:underline"}
                            >@{room_info?.username}</TextLink>
                            <Subheading>{room_info?.description}</Subheading>
                        </div>
                    </SidebarHeader>
                    <SidebarBody className={"justify-between"}>
                        <div>
                            <Dropdown>
                                <DropdownButton as={SidebarItem} className="lg:mb-2.5">
                                    <SidebarLabel
                                        className={"flex grow gap-2 items-center text-lg font-medium hover:cursor-pointer"}>
                                        <TvOne theme="outline" size="24" fill="#333"/>
                                        总监的同事们
                                    </SidebarLabel>
                                    <ChevronDownIcon/>
                                </DropdownButton>
                                <DropdownMenu className="min-w-80 lg:min-w-64" anchor="bottom start">
                                    {streamers.map((streamer) => {
                                        return streamer.id != 0 ?
                                            <DropdownItem key={streamer.id} href={"/" + streamer.room_id.toString()}>
                                                <Avatar
                                                    src={streamer.small_avatar}
                                                    className={"min-w-12 min-h-12"}
                                                    square/>
                                                <DropdownLabel className={"text-lg"}>{streamer.nickname}</DropdownLabel>
                                            </DropdownItem> : null
                                    })}
                                </DropdownMenu>
                            </Dropdown>
                            <SidebarSection className={"mt-3"}>
                                <SidebarItem href={"/checker"}
                                             className={"flex justify-between gap-2 items-center font-medium"}>
                                    {/*<FontAwesomeIcon icon={faSearchengin} className={"size-8"}/>*/}
                                    <Search theme="outline" size="24" fill="#333"/>
                                    <SidebarLabel className={"ml-4 text-lg"}>查成分</SidebarLabel>
                                </SidebarItem>
                                <SidebarItem href={"/block_user"}
                                             className={"flex justify-between gap-2 items-center font-medium"}>
                                    <Bug theme="outline" size="24" fill="#333"/>
                                    <SidebarLabel className={"ml-4 text-lg"}>四害榜</SidebarLabel>
                                </SidebarItem>
                            </SidebarSection>
                        </div>
                        <div>
                            {room_id != "22747736" ?
                                <SidebarSection>
                                    <SidebarItem href={"/22747736"} className={"text-base font-medium"}>
                                        <BackOne theme="outline" size="24" fill="#333"/>
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
            {children}
        </SidebarLayout>

    );
}