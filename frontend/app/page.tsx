'use client'
import {useEffect} from "react";
import {useRouter} from "next/navigation";

export default function Home() {
    const router = useRouter();
    useEffect(() => {
        router.push('/22747736');
    }, []);

    return null;  // 空的组件，因为页面会被重定向
}