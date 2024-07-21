'use client'
import React, {createContext, useContext, useEffect, useState} from "react";
import {streamers} from "@/data/streamers";

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

interface RoomContextProps {
    id: number,
    roomInfo: streamer
    toggleRoom: (id: number) => void
}

const RoomContext = createContext<RoomContextProps | undefined>(undefined);
const streamerData: streamer[] = streamers;

export const RoomProvider = ({children}: {children: React.ReactNode}) => {
    const [id, setId] = useState<number>(0);
    const [roomInfo, setRoomInfo] = useState<streamer>(streamerData[0]);

    useEffect(() => {
        const selectedStreamer = streamers[id];
        if (selectedStreamer) {
            setRoomInfo(selectedStreamer);
        }
    }, [id]);

    const toggleRoom = (id: number) => {
        setId(id);
    };
        return (
            <RoomContext.Provider value={{id, roomInfo, toggleRoom}}>
                {children}
            </RoomContext.Provider>
        );
}

export const useRoom = () => {
    const context = useContext(RoomContext);
    if (!context) {
        throw new Error('useRoom must be used within a RoomProvider');
    }
    return context
}