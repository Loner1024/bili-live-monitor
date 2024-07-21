import {ArrowPathIcon} from "@heroicons/react/24/solid";
import React from "react";

export const Loading = () => {
    return (
        <div className={"flex justify-center items-center h-96 w-full"}>
            <div className={"flex justify-center items-center size-1/12"}>
                <ArrowPathIcon className={"animate-spin text-gray-500"}/>
            </div>
        </div>
    )
}
