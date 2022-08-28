import { USER_SERVICE_URL } from "../constatns";
import { client } from "./apiClient"

interface Profile {
    firstName: string;
    lastName: string;
}

export interface RegisterUserRequest {
    email: string;
    password: string;
    profile: Profile;
} 

export const registerUser = async (user: RegisterUserRequest) => {
    let resp = await client.post(`${USER_SERVICE_URL}`, { ...user })
    return resp;
}

export const searchUsers = async (keyword: string) =>  {
    let resp = await client.get(`${USER_SERVICE_URL}/search/${keyword}`);
    return resp.data;
}