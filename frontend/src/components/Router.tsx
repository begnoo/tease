import { Route, Routes } from "react-router-dom";
import {
  HOME_PAGE, INIT_SOURCE_PAGE, LOGIN_PAGE, REGISTER_PAGE, SOURCE_OVERVIEW_PAGE, SOURCE_PAGE, SOURCE_USERVIEW_PAGE,
} from "../constatns";
import HomePage from "../pages/HomePage";
import LoginPage from "../pages/LoginPage";
import RegisterPage from "../pages/RegisterPage";
import InitPage from "../pages/source/InitPage";
import AllSourcesPage from "../pages/source/AllSourcesPage";
import UserSourcesPage from "../pages/source/UserSourcesPage";
import SourcePage from "../pages/source/SourcePage";

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
        element={<AllSourcesPage/>}
      />
      <Route
        path={SOURCE_USERVIEW_PAGE}
        element={<UserSourcesPage/>}
      />
      <Route
        path={INIT_SOURCE_PAGE}
        element={<InitPage/>}
      />
      <Route
        path={SOURCE_PAGE}
        element={<SourcePage/>}
      />
    </Routes>
  );
}