import { format, fromUnixTime, parseISO } from 'date-fns';
import formatDistance from 'date-fns/formatDistance';

export const howMuchAgo = (date_string: string | undefined): string => {
    if (!date_string) {
        return "";
    }
    const date = parseISO(date_string);
    const distance = formatDistance(new Date(), date);
    return `${distance} ago`;
}

export const timeDistance = (date_string: string | undefined): string => {
    if (!date_string) {
        return "";
    }
    const date = parseISO(date_string);
    const distance = formatDistance(new Date(), date);
    return distance;
}

export const fromMilis = (milis: number | undefined): string => {
    if (!milis) {
        return "";
    }
    const date = fromUnixTime(milis);
    return format(date, "dd MMM yyyy")
}

export const fromMilisTime = (milis: number | undefined): string => {
    if (!milis) {
        return "";
    }
    const date = fromUnixTime(milis);
    return format(date, "HH:mm")
}