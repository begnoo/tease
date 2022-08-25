import { Route, Routes } from "react-router-dom";
import {
  HOME_PAGE, LOGIN_PAGE,
} from "../constatns";
import HomePage from "../pages/HomePage";
import LoginPage from "../pages/LoginPage";

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
    </Routes>
  );
}