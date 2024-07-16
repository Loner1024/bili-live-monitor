'use client'
import {Avatar} from '@/components/avatar'
import {
    Dropdown,
    DropdownButton,
    DropdownDivider,
    DropdownItem,
    DropdownLabel,
    DropdownMenu,
} from '@/components/dropdown'
import {Navbar, NavbarItem, NavbarSection, NavbarSpacer} from '@/components/navbar'
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
import {
    ArrowRightStartOnRectangleIcon,
    ChevronDownIcon,
    Cog8ToothIcon,
    LightBulbIcon,
    ShieldCheckIcon,
    UserIcon,
} from '@heroicons/react/16/solid'
import {InboxIcon, MagnifyingGlassIcon,} from '@heroicons/react/20/solid'

import {FontAwesomeIcon} from '@fortawesome/react-fontawesome'
import {faBilibili} from "@fortawesome/free-brands-svg-icons";
import {useState} from "react";
import {streamers} from "@/data/streamers";
import {faBackward} from "@fortawesome/free-solid-svg-icons";

interface streamer {
    id: number,
    nickname: string,
    username: string,
    bilibili_link: string,
    avatar: string,
    small_avatar: string,
    description: string,
}

export default function Home() {
    const streamerData: streamer[] = streamers;
    const [selectedId, setSelectedId] = useState<number>(0);

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
                            <NavbarItem href="/search" aria-label="Search">
                                <MagnifyingGlassIcon/>
                            </NavbarItem>
                            <NavbarItem href="/inbox" aria-label="Inbox">
                                <InboxIcon/>
                            </NavbarItem>
                            <Dropdown>
                                <DropdownButton as={NavbarItem}>
                                    <Avatar src="/profile-photo.jpg" square/>
                                </DropdownButton>
                                <DropdownMenu className="min-w-64" anchor="bottom end">
                                    <DropdownItem href="/my-profile">
                                        <UserIcon/>
                                        <DropdownLabel>My profile</DropdownLabel>
                                    </DropdownItem>
                                    <DropdownItem href="/settings">
                                        <Cog8ToothIcon/>
                                        <DropdownLabel>Settings</DropdownLabel>
                                    </DropdownItem>
                                    <DropdownDivider/>
                                    <DropdownItem href="/privacy-policy">
                                        <ShieldCheckIcon/>
                                        <DropdownLabel>Privacy policy</DropdownLabel>
                                    </DropdownItem>
                                    <DropdownItem href="/share-feedback">
                                        <LightBulbIcon/>
                                        <DropdownLabel>Share feedback</DropdownLabel>
                                    </DropdownItem>
                                    <DropdownDivider/>
                                    <DropdownItem href="/logout">
                                        <ArrowRightStartOnRectangleIcon/>
                                        <DropdownLabel>Sign out</DropdownLabel>
                                    </DropdownItem>
                                </DropdownMenu>
                            </Dropdown>
                        </NavbarSection>
                    </Navbar>
                }
                sidebar={
                    <Sidebar>
                        <SidebarHeader>
                            <div className={"flex flex-col gap-1"}>
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
            </SidebarLayout>
        </div>

    );
}