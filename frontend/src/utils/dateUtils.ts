import { parseISO } from 'date-fns';
import formatDistance from 'date-fns/formatDistance';

export const howMuchAgo = (date_string: string): string => {
    const date = parseISO(date_string);
    const distance = formatDistance(new Date(), date);
    return `${distance} ago`;
}