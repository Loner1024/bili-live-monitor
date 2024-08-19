import {DataTable} from "@/components/func/DataTable"
import React from "react";
import MySidebar from "@/components/func/sidebar";


export default function Home(params: { params: { room_id: string } }) {
    const room_id = params.params.room_id;
    return (
        <MySidebar room_id={room_id}>
            <DataTable room_id={room_id}/>
        </MySidebar>
    );
}

// TODO: from api get data
export async function generateStaticParams() {
    return [
        {room_id: "22747736"},
        {room_id: "23649609"},
        {room_id: "21533102"},
        {room_id: "14733388"},
    ]
}