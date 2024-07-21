import {format, fromUnixTime} from "date-fns";

export function getFormatTime(timestamp: number) {
    return format(fromUnixTime(timestamp), 'yyyy-MM-dd HH:mm:ss');
}

export function getTimestampSecs(date: { getTime: () => number }) {
    return parseInt(String(date.getTime() / 1000))
}
