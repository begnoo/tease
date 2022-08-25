import { AUTH_SERVICE_URL } from "../constatns";
import { client } from "./apiClient"

interface Credidentials {
    email: string;
    password: string;
} 

export const loginUser = async (creds: Credidentials) => {
    let resp = await client.post(`${AUTH_SERVICE_URL}/login`, { ...creds })
    return resp;
}