import MainPage from "./components/MainPage";
import Login from "./pages/login/Login";
import { appWindow } from "@tauri-apps/api/window";
export default function App(){
    let window;
    switch (appWindow.label){
        case 'login':
            window=<Login />;
            break;
        default:
            window=<MainPage />
    }
    console.log(appWindow.label)
    return(window)
}