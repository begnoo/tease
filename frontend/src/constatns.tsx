export const HOME_PAGE: string = "/";
export const LOGIN_PAGE: string = "/login";
export const REGISTER_PAGE: string = "/register";

export const AUTH_SERVICE_URL = process.env.REACT_APP_AUTH_SERVICE;
export const USER_SERVICE_URL = process.env.REACT_APP_USER_SERVICE;

interface TeaseRoute {
    href: string,
    name: string
}

export const ROUTES: TeaseRoute[] = [
    { href: HOME_PAGE, name: "HOME" },
];

export const CRED_ROUTES: TeaseRoute[] = [
    { href: LOGIN_PAGE, name: "LOGIN" },
    { href: REGISTER_PAGE, name: "REGISTER" },
];