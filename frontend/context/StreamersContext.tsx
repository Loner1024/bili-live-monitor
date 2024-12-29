'use client'
import React, {createContext, ReactNode, useContext, useEffect, useMemo, useState} from 'react';

interface StreamersContextProps {
    streamers: Streamers[];
}

type StreamersResponse = {
    code: number
    message: string
    data: Streamers[]
}

type Streamers = {
    id: number;
    nickname: string
    username: string
    bilibili_link: string
    room_id: number
    avatar: string
    small_avatar: string
    description: string
}

const fetcher = async (): Promise<StreamersResponse> => {
    const response = await fetch(`${process.env.API_URL}/api/streamers`);

    if (!response.ok) {
        throw new Error('Network response was not ok');
    }

    return response.json();
};


const StreamersContext = createContext<StreamersContextProps | undefined>(undefined);

export const StreamerProvider = ({children}: { children: ReactNode }) => {
    const [streamers, setStreamers] = useState<Streamers[]>([]);
    useEffect(() => {
        if (sessionStorage.getItem("streamers")) {
            setStreamers(JSON.parse(sessionStorage.getItem("streamers")!));
        } else {
            fetcher().then((response) => {
                setStreamers(response.data);
                sessionStorage.setItem("streamers", JSON.stringify(response.data));
            });
        }
    }, []);


    return (
        <StreamersContext.Provider value={{streamers}}>
                {children}
        </StreamersContext.Provider>
    );
};

export const useStreamer = () => {
    const context = useContext(StreamersContext);
    if (!context) {
        throw new Error('useTheme must be used within a ThemeProvider');
    }
    return context;
};
