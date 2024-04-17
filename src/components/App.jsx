import { useState } from "react";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";
//import reactLogo from "./assets/react.svg";
import "./App.css";
import SelectClientes from "./SelectClientes";
import CuadroPrincipal from "./CuadroPrincipal";
import ResumenPago from "./ResumenPago";
import TablaProductos from "./TablaProductos";
let mensaje1;
let vacia;
let user;
let posA = true;
let posicionVenta = 0;
let timeoutId;
let codigosProv = [];
let codigosProd = [];
let configs;
let idUlt;
let buscador;
let beep = new Audio('src/assets/beep.mp3');
let error = new Audio('src/assets/error.mp3');
let productosDib = [];
let productosVentaAct = [];
beep.volume = 1;
error.volume = 0.2;


async function open_login() {
  return await invoke("open_login");
}

open_login();

function App() {
  const [logged, setLogged] = useState(false);
  const [prodFoc, setProdFoc] = useState(true);
  const [pos, setPos] = useState(true);
  const [venta, setVenta] = useState();
  const [configs, setConfigs] = useState();
  const [busqueda, setBusqueda] = useState();
  const [focuseado, setFocuseado] = useState(0);
  const [rend, setRend] = useState(<>
    <section id="no-iniciado" className="main-screen">
      <p>
        Se requiere inicio de sesión
      </p>
    </section>
  </>);
  function handleFocuseado(e) {
    if (e.currentTarget.value && e.currentTarget.value != "") {
      if (e.keyCode == 40 || e.keyCode == 38 || e.keyCode == 13) {
        e.preventDefault();
        if (e.keyCode == 40 && focuseado < configs.cantidad_productos) {
          setFocuseado(focuseado + 1);
        } else if (e.keyCode == 38 && focuseado > 0) {
          setFocuseado(focuseado - 1);
        }
      } else if (e.keyCode == 27) {
        e.currentTarget.value = "";
        setBusqueda(e.currentTarget.value);
      
      }
    }
    if (e.currentTarget.value == ""){
      setFocuseado(0)
    }
  }

  function draw() {
    if (logged) {
      get_configs().then(conf => {
        get_venta_actual(pos).then(sale => {
          setVenta(sale);
          setConfigs(conf)
          setRend(<>
            <header className="container" >
              <section id="header">
                <div>
                  <form autoComplete="off">
                    <input type="text" id="buscador" placeholder="Buscar producto.." onKeyDown={(e) => { handleFocuseado(e) }} onClick={() => { isProd(true) }} onChange={(e) => { setBusqueda(e.currentTarget.value) }} />
                  </form>
                </div>
                <div>
                  <SelectClientes />
                </div>
              </section>
            </header>
            <main className="main-screen">
              <CuadroPrincipal venta={sale} conf={conf} prodFoc={prodFoc} posSet={setPos} isProd={isProd} busqueda={busqueda} focuseado={focuseado} />
              <ResumenPago pos={pos} venta={sale} configs={conf} prodFoc={prodFoc} isProd={isProd} />

            </main>
          </>);
        });
      });
    }
    async function get_venta_actual(pos) {
      let res = await invoke("get_venta_actual", { pos: pos });
      return res;
    }
    async function get_configs() {
      return await invoke("get_configs");
    }
  }
  useEffect(() => draw(), [logged, prodFoc, busqueda,focuseado])
  function isProd(val) {
    setProdFoc(val)
  }
  async function unlisten() {

    return await listen('main', (pl) => {
      if (pl.payload.message == 'dibujar venta') {
        get_venta_actual().then(venta => setVenta(venta));
      } else if (pl.payload.message == "confirm stash") {
        // open_confirm_stash(pos)
      } else if (pl.payload.message == "inicio sesion") {
        setLogged(true);
      } else if (pl.payload.message == "cerrar sesion") {
        cerrar_sesion()
      } else if (pl.payload.message == "open stash") {
        open_stash()
      }
    })
  }




  unlisten();
  return (
    rend
  );

}


export default App;