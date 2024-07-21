'use client'
import {Avatar} from '@/components/avatar'
import {Dropdown, DropdownButton, DropdownItem, DropdownLabel, DropdownMenu,} from '@/components/dropdown'
import {Navbar, NavbarSection, NavbarSpacer} from '@/components/navbar'
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
import {FontAwesomeIcon} from '@fortawesome/react-fontawesome'
import {faBilibili, faSearchengin} from "@fortawesome/free-brands-svg-icons";
import React from "react";
import {faBackward} from "@fortawesome/free-solid-svg-icons";
import {useTheme} from '@/context/ThemeContext';
import {ChevronDownIcon, MoonIcon, SunIcon} from "@heroicons/react/24/solid";
import {useRoom} from "@/context/RoomContext";
import {usePathname} from 'next/navigation';


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

export const ApplicationLayout = ({children}: { children: React.ReactNode }) => {
    const currentPath = usePathname();
    const {theme, toggleTheme} = useTheme();
    const {id, roomInfo, toggleRoom} = useRoom();

    console.log(currentPath)

    const handleClick = (id: number) => {
        toggleRoom(id)
    };

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
                                    src={roomInfo.avatar}/>
                                <Heading>{roomInfo.nickname}</Heading>
                                <TextLink
                                    href={roomInfo.bilibili_link}
                                    target={"_blank"}
                                    className={"no-underline hover:underline"}
                                >@{roomInfo.username}</TextLink>
                                <Subheading>{roomInfo.description}</Subheading>
                            </div>
                        </SidebarHeader>
                        <SidebarBody className={"justify-between"}>
                            <div>
                            <Dropdown>
                                <DropdownButton as={SidebarItem} className="lg:mb-2.5">
                                    <SidebarLabel className={"flex grow gap-2 items-center text-lg font-medium hover:cursor-pointer"}>
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
                                    <SidebarSection className={"mt-3"}>
                                        <SidebarItem href={"/checker"} className={"flex grow gap-2 items-center font-medium"}>
                                            <FontAwesomeIcon icon={faSearchengin} size={"2xs"} className={"size-8"} />
                                            <SidebarLabel className={"ml-4 text-lg"}>查成分</SidebarLabel>
                                        </SidebarItem>
                                    </SidebarSection>
                            </div>
                            <div>
                                {id != 0 ? <SidebarSection>
                                    <SidebarItem onClick={() => handleClick(0)} className={"text-base font-medium"}>
                                        <FontAwesomeIcon icon={faBackward} className={"size-6"}/>
                                        <SidebarLabel className={"ml-4"}>回去看卢</SidebarLabel>
                                    </SidebarItem>
                                </SidebarSection> : null}
                                {currentPath == "/checker" ? <SidebarSection>
                                    <SidebarItem href={"/"} className={"text-base font-medium"}>
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
                {children}
            </SidebarLayout>
    );
}


