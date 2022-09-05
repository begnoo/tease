export const HOME_PAGE: string = "/";
export const LOGIN_PAGE: string = "/login";
export const REGISTER_PAGE: string = "/register";
export const SOURCE_OVERVIEW_PAGE: string = "/source/all";
export const SOURCE_USERVIEW_PAGE: string = "/source/:user";
export const INIT_SOURCE_PAGE: string = "/source/init";
export const SOURCE_PAGE: string = "/source/:user/:source";
export const SOURCE_COLLABS_PAGE: string = "/source/:user/:source/collabs";
export const COLLABS_PAGE: string = "/collabs";
export const COMMITS_PAGE: string = "/source/:user/:source/commits/:branch";
export const DIFF_PAGE: string = "/source/:user/:source/commits/diff/:commit";
export const STATS_PAGE: string = "/source/:user/:source/stats";

export const AUTH_SERVICE_URL = process.env.REACT_APP_AUTH_SERVICE;
export const USER_SERVICE_URL = process.env.REACT_APP_USER_SERVICE;
export const SOURCE_SERVICE_URL = process.env.REACT_APP_SOURCE_SERVICE;
export const COLLAB_SERVICE_URL = process.env.REACT_APP_COLLAB_SERVICE;
export const SOURCE_STORAGE_URL = process.env.REACT_APP_STORAGE_SERVICE;
export const SOURCE_CLONE_URL = process.env.REACT_APP_STORAGE_CLONE;
export const STATS_SERVICE_URL = process.env.REACT_APP_STATS_SERVICE;


interface TeaseRoute {
    href: string,
    name: string
}

export const ROUTES: TeaseRoute[] = [
    { href: HOME_PAGE, name: "HOME" },
    { href: SOURCE_OVERVIEW_PAGE, name: "SOURCES" },
];

export const AUTH_ROUTES: TeaseRoute[] = [
    { href: INIT_SOURCE_PAGE, name: "INIT" },
    { href: COLLABS_PAGE, name: "COLLABS" },
];

export const CRED_ROUTES: TeaseRoute[] = [
    { href: LOGIN_PAGE, name: "LOGIN" },
    { href: REGISTER_PAGE, name: "REGISTER" },
];