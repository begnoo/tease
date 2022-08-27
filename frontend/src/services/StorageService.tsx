import { SOURCE_STORAGE_URL } from "../constatns";
import { client } from "./apiClient"

export interface Item {
    dtype: string;
    name: string;
    sha1: string;
} 

export interface ReadBranchRequest {
    user: string | undefined,
    source: string | undefined,
}

export interface ReadDataRequest {
    sha: string | undefined,
    user: string | undefined,
    source: string | undefined,
}

export interface Blob {
    size: number,
    content: string,
}

export interface BranchContent {
    name: string,
    sha: string
}

export const readTree = async ({sha, user, source}: ReadDataRequest) => {
    let resp = await client.get(`${SOURCE_STORAGE_URL}/${user}/${source}/tree/${sha}`);
    let data: Item[] = resp.data.items;
    return data;
}

export const readBlob = async ({sha, user, source}: ReadDataRequest) => {
    let resp = await client.get(`${SOURCE_STORAGE_URL}/${user}/${source}/blob/${sha}`);
    let data: Blob = resp.data;
    return data;
}

export const readBranches = async ({user, source}: ReadBranchRequest) => {
    let resp = await client.get(`${SOURCE_STORAGE_URL}/${user}/${source}/branch`);
    let data: BranchContent[] = resp.data;
    return data;
}