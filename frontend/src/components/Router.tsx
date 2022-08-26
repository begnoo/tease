import { Route, Routes } from "react-router-dom";
import {
  HOME_PAGE, LOGIN_PAGE, REGISTER_PAGE, SOURCE_OVERVIEW_PAGE, SOURCE_USERVIEW_PAGE,
} from "../constatns";
import HomePage from "../pages/HomePage";
import LoginPage from "../pages/LoginPage";
import RegisterPage from "../pages/RegisterPage";
import OverviewPage from "../pages/source/OverviewPage";
import UserViewPage from "../pages/source/UserViewPage";

export default function Router(): JSX.Element {
  return (
    <Routes>
      <Route
        path={HOME_PAGE}
        element={<HomePage/>}
      />
      <Route
        path={LOGIN_PAGE}
        element={<LoginPage/>}
      />
      <Route
        path={REGISTER_PAGE}
        element={<RegisterPage/>}
      />
      <Route
        path={SOURCE_OVERVIEW_PAGE}
        element={<OverviewPage/>}
      />
    <Route
        path={SOURCE_USERVIEW_PAGE}
        element={<UserViewPage/>}
      />
    </Routes>
  );
}