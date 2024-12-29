// 'use client'
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