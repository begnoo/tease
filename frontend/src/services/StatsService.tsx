import { STATS_SERVICE_URL } from "../constatns";
import { client } from "./apiClient"

export interface ReadCommitsStats {
    user: string | undefined,
    source: string | undefined,
}

export interface CommitStats {
    id: string,
	created_at: number,
	added: number,
	deleted: number,
	owner: string,
	user: string,
	source: string,
	sha: string,
}

export interface CommitStatsByDay {
	id: string
	count: number
	added: number
	deleted: number
}

export interface CommitStatsByCollab {
	user: string
	count: number
	added: number
	deleted: number
}

export interface CommitDateAndCount {
	date: string
	added: number
	deleted: number
	count: number
}

export interface CommitStatsByCollabAndDate {
	user: string
	added: number
	deleted: number
	items: CommitDateAndCount[]
}
export const readCommitsStatsByDate = async ({user, source}: ReadCommitsStats) => {
    let resp = await client.get(`${STATS_SERVICE_URL}/commits/${user}/${source}`);
    let data: CommitStatsByDay[] = resp.data;
    return data;
}

export const readCommitsStatsByUser = async ({user, source}: ReadCommitsStats) => {
    let resp = await client.get(`${STATS_SERVICE_URL}/commits/${user}/${source}/by-user`);
    let data: CommitStatsByCollab[] = resp.data;
    return data;
}

export const readCommitsStatsByUserAndDate = async ({user, source}: ReadCommitsStats) => {
    let resp = await client.get(`${STATS_SERVICE_URL}/commits/${user}/${source}/by-user-and-date`);
    let data: CommitStatsByCollabAndDate[] = resp.data;
	console.log(data);
    return data;
}