import axios, { HeadersDefaults } from "axios";
import { getUserFromLocalStorage } from "../utils/jwtUtils";

const client = axios.create({
  headers: {
    "Content-type": "application/json",
  }
});

client.interceptors.request.use(function (config) {
  const user = getUserFromLocalStorage();

  let token = "";
 
  if (user !== null) {
    token = user.token;
  }
  
  config.headers = {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${token}`
  }

  return config;
});

export interface BackendError {
  error: string | []
}


export { client };