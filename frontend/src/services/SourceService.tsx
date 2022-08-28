import { SOURCE_SERVICE_URL } from "../constatns";
import { client } from "./apiClient"

export interface Source {
    id: number,
    createdAt: string,
    name: string,
    owner: string,
    description: string,
    visability: boolean,
}

export interface InitSourceRequest {
    name: string,
    owner: string,
    description: string,
    visability: boolean,
}

export interface AddCollabRequest {
    name: string | undefined,
    owner: string | undefined,
    collabarator: string | undefined,
}

export interface Collab {
    id: number | undefined,
    name: string | undefined,
    reactedToInvite: boolean | undefined,
    acceptedInvite: boolean | undefined,
    expiersAt: string | undefined,
}

export const readSources = async (): Promise<Source[]> => {
    let resp = await client.get(`${SOURCE_SERVICE_URL}`);
    let data: Source[] = resp.data;
    return data;
}

export const readSourcesByUser = async (owner: string | undefined): Promise<Source[]> => {
    let resp = await client.get(`${SOURCE_SERVICE_URL}?owner=${owner}`);
    let data: Source[] = resp.data;
    return data;
}

export const initSource = async (initRequest: InitSourceRequest | undefined): Promise<Source[]> => {
    let resp = await client.post(`${SOURCE_SERVICE_URL}`, initRequest);
    let data: Source[] = resp.data;
    return data;
}

export const addCollab = async (req: AddCollabRequest | undefined) => {
    let resp = await client.post(`${SOURCE_SERVICE_URL}/collabs/add`, req);
    return resp;
}

export const getCollabs = async (owner: string | undefined, source: string | undefined): Promise<Collab[]> => {
    let resp = await client.get(`${SOURCE_SERVICE_URL}/collabs/${owner}/${source}`);
    const data: Promise<Collab[]> = resp.data;
    console.log(data);
    return data;
}