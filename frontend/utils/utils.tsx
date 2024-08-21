import {format, fromUnixTime} from "date-fns";

export function getFormatTime(timestamp: number) {
    return format(fromUnixTime(timestamp), 'yyyy-MM-dd HH:mm:ss');
}

export function getTimestampSecs(date: { getTime: () => number }) {
    return parseInt(String(date.getTime() / 1000))
}


export class StatisticsResult {
    timestamp: number = 0
    danmu_total: number = 0      // 总弹幕数量
    danmu_people: number = 0     // 总弹幕人数
    super_chat_total: number = 0 // 总SC数量
    super_chat_worth: number = 0 // 总SC人数
}

