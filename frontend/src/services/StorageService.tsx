import { SOURCE_STORAGE_URL } from "../constatns";
import { client } from "./apiClient"

export interface Item {
    dtype: string;
    name: string;
    sha1: string;
} 

export interface CommitItem {
    sha1: string,
    date: number,
    author: string,
    message: string,
    parents: String[],
} 

export interface DiffItem {
    path: string,
    diff: String[],
    added: number,
    deleted: number
} 

export interface ReadBranchRequest {
    user: string | undefined,
    source: string | undefined,
}

export interface ReadCommitsRequest {
    user: string | undefined,
    source: string | undefined,
    branch: string | undefined,
}

export interface ReadCommitRequest {
    user: string | undefined,
    source: string | undefined,
    commit: string | undefined,
}

export interface DiffCommitsRequest {
    user: string | undefined,
    source: string | undefined,
    commit: string | undefined,
    parentCommit: string | undefined
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
    tree_sha1: string,
    commit: CommitItem
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

export const readCommits = async ({user, source, branch}: ReadCommitsRequest) => {
    let resp = await client.get(`${SOURCE_STORAGE_URL}/${user}/${source}/commits/branch/${branch}`);
    let data: CommitItem[] = resp.data.items;
    return data;
}

export const readCommit = async ({user, source, commit}: ReadCommitRequest): Promise<CommitItem> => {
    let resp = await client.get(`${SOURCE_STORAGE_URL}/${user}/${source}/commits/commit/${commit}`);
    let data: CommitItem = resp.data;
    return data;
}

export const diffCommits = async ({user, source, commit, parentCommit}: DiffCommitsRequest) => {
    let resp = await client.post(`${SOURCE_STORAGE_URL}/${user}/${source}/diff`, {commit, parentCommit});
    let data: DiffItem[] = resp.data.items;
    return data;
}