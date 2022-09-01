import { parseISO } from 'date-fns';
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