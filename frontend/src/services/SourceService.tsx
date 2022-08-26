import { SOURCE_SERVICE_URL, USER_SERVICE_URL } from "../constatns";
import { client } from "./apiClient"

export interface Source {
    id: number,
    createdAt: string,
    name: string,
    owner: string,
}

export const readSources = async (): Promise<Source[]> => {
    let resp = await client.get(`${SOURCE_SERVICE_URL}`);
    let data: Source[] = resp.data;
    return data;
}

export const readSourcesByUser = async (owner: string | undefined): Promise<Source[]> => {
    let resp = await client.get(`${SOURCE_SERVICE_URL}?owner=${owner}`);
    let data: Source[] = resp.data;
    console.log(data);
    return data;
}