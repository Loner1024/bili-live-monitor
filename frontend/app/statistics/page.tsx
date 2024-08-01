import React from 'react';
import {Danmu_chart} from "@/components/danmu_chart";
import {ScWorthChart} from "@/components/sc_worth_chart";
import {DanmuPeopleChart} from "@/components/danmu_people_chart";

const data = {
    "code": 0,
        "message": "success",
        "data": [
        {
            "timestamp": 1722355200,
            "danmu_total": 10957,
            "danmu_people": 1534,
            "super_chat_total": 2,
            "super_chat_worth": 60
        },
        {
            "timestamp": 1722268800,
            "danmu_total": 121202,
            "danmu_people": 6932,
            "super_chat_total": 84,
            "super_chat_worth": 3030
        },
        {
            "timestamp": 1722182400,
            "danmu_total": 131379,
            "danmu_people": 7265,
            "super_chat_total": 79,
            "super_chat_worth": 2890
        },
        {
            "timestamp": 1722096000,
            "danmu_total": 114835,
            "danmu_people": 6987,
            "super_chat_total": 71,
            "super_chat_worth": 2430
        },
        {
            "timestamp": 1722009600,
            "danmu_total": 94489,
            "danmu_people": 6402,
            "super_chat_total": 69,
            "super_chat_worth": 2210
        }
    ]
}

const Page = () => {
    return (
        <div className={""}>
            <Danmu_chart chartData={data.data.reverse()}/>
            <ScWorthChart chartData={data.data}/>
            <DanmuPeopleChart chartData={data.data}/>
        </div>
    );
};

export default Page;