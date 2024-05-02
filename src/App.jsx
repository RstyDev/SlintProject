import MainPage from "./components/MainPage";
import LoginPage from "./pages/LoginPage";
import ConfirmPage from "./pages/ConfirmPage";
import { appWindow } from "@tauri-apps/api/window";
export default function App(){
    let window;
    switch (appWindow.label){
        case 'login':
            window=<LoginPage />;
            break;
        default:
            window=<MainPage />
    }
    console.log(appWindow.label)
    return(window)
}