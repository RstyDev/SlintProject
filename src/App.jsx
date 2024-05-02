import MainPage from "./pages/MainPage";
import LoginPage from "./pages/LoginPage";
import ConfirmPage from "./pages/ConfirmPage";
import AddProdPage from "./pages/AddProdPage";
import AddProvPage from "./pages/AddProvPage";
import AddUserPage from "./pages/AddUserPage";
import AddCliPage from "./pages/AddCliPage";
import CerrarCajaPage from "./pages/CerrarCajaPage";
import SelectAmountPage from "./pages/SelectAmountPage";
import EditSettingsPage from "./pages/EditSettingsPage";
import OpenStashPage from "./pages/OpenStashPage";

import { appWindow } from "@tauri-apps/api/window";

export default function App() {
    let window;
    switch (appWindow.label) {
        case "login":
            window = <LoginPage />;
            break;
        case "add-prod":
            window = <AddProdPage />;
            break;
        case "add-prov":
            window = <AddProvPage />;
            break;
        case "add-user":
            window = <AddUserPage />;
            break;
        case "add-cliente":
            window = <AddCliPage />;
            break;
        case "cerrar-caja": //TODO jsx
            window = <CerrarCajaPage />;
            break;
        case "edit-settings": //TODO jsx
            window = <EditSettingsPage />;
            break;
        case "select-amount":
            window = <SelectAmountPage />;
            break;
        case "open-stash": //TODO jsx
            window = <OpenStashPage />;
            break;
        case "confirm-cancel": //TODO parcial
            window = <ConfirmPage message={"Desea cancelar la venta?"} />;
            break;
        case "confirm-stash": //TODO parcial
            window = <ConfirmPage message={"Desea guardar la venta?"} />;
            break;
        default:
            window = <MainPage />;
            break;
    }
    console.log(appWindow.label)
    return (window)
}