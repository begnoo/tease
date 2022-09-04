import { Route, Routes } from "react-router-dom";
import {
  COLLABS_PAGE,
  COMMITS_PAGE,
  DIFF_PAGE,
  HOME_PAGE, INIT_SOURCE_PAGE, LOGIN_PAGE, REGISTER_PAGE, SOURCE_COLLABS_PAGE, SOURCE_OVERVIEW_PAGE, SOURCE_PAGE, SOURCE_USERVIEW_PAGE, STATS_PAGE,
} from "../constatns";
import HomePage from "../pages/HomePage";
import LoginPage from "../pages/LoginPage";
import RegisterPage from "../pages/RegisterPage";
import InitPage from "../pages/source/InitPage";
import AllSourcesPage from "../pages/source/AllSourcesPage";
import UserSourcesPage from "../pages/source/UserSourcesPage";
import SourcePage from "../pages/source/SourcePage";
import SourceCollabPage from "../pages/source/SourceCollabPage";
import CollabPage from "../pages/collab/CollabPage";
import CommitsPage from "../pages/commits/CommitsPage";
import DiffPage from "../pages/diffs/DiffPage";
import StatsPage from "../pages/source/StatsPage";

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
      <Route
        path={SOURCE_COLLABS_PAGE}
        element={<SourceCollabPage/>}
      />
      <Route
        path={COLLABS_PAGE}
        element={<CollabPage/>}
      />
      <Route
        path={COMMITS_PAGE}
        element={<CommitsPage/>}
      />
      <Route
        path={DIFF_PAGE}
        element={<DiffPage/>}
      />
      <Route
        path={STATS_PAGE}
        element={<StatsPage/>}
      />
    </Routes>
  );
}