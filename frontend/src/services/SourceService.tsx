import { COLLAB_SERVICE_URL, SOURCE_SERVICE_URL } from "../constatns";
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
    from: string | undefined,
    sourceName: string | undefined,
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

export const deleteSource = async (id: number | undefined): Promise<Source> => {
    let resp = await client.delete(`${SOURCE_SERVICE_URL}/${id}`);
    const data: Promise<Source> = resp.data;
    return data;
}

export const addCollab = async (req: AddCollabRequest | undefined) => {
    let resp = await client.post(`${COLLAB_SERVICE_URL}/add`, req);
    return resp;
}

export const getCollabs = async (owner: string | undefined, source: string | undefined): Promise<Collab[]> => {
    let resp = await client.get(`${COLLAB_SERVICE_URL}/${owner}/source/${source}`);
    const data: Promise<Collab[]> = resp.data;
    return data;
}

export const getCollabsByName = async (name: string | undefined): Promise<Collab[]> => {
    let resp = await client.get(`${COLLAB_SERVICE_URL}/${name}/by-name`);
    const data: Promise<Collab[]> = resp.data;
    return data;
}

export const deleteCollab = async (id: number | undefined): Promise<Collab> => {
    let resp = await client.delete(`${COLLAB_SERVICE_URL}/${id}`);
    const data: Promise<Collab> = resp.data;
    return data;
}

export const acceptCollab = async (id: number | undefined): Promise<Collab> => {
    let resp = await client.post(`${COLLAB_SERVICE_URL}/${id}/accept`);
    const data: Promise<Collab> = resp.data;
    return data;
}

export const rejectCollab = async (id: number | undefined): Promise<Collab> => {
    let resp = await client.post(`${COLLAB_SERVICE_URL}/${id}/reject`);
    const data: Promise<Collab> = resp.data;
    return data;
}