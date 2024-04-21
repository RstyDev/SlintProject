import { useState } from "react";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";
//import reactLogo from "./assets/react.svg";
import "./App.css";
import SelectClientes from "./SelectClientes";
import CuadroPrincipal from "./CuadroPrincipal";
import ResumenPago from "./ResumenPago";
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

async function get_configs() {
  return await invoke("get_configs");
}
async function buscarProducto(filtrado) {
  return await invoke("get_productos_filtrado", { filtro: '' + filtrado });
}
async function open_login() {
  return await invoke("open_login");
}
async function get_log_state() {
  return await invoke("get_log_state");
}
async function agregarProdVentaAct(prod,pos) {
  return await invoke("agregar_producto_a_venta", { prod: prod, pos: pos });
}

async function incrementarProdVentaAct(index,pos) {
  return await invoke("incrementar_producto_a_venta", { index: index, pos: pos });
}
async function descontarProdVentaAct(index,pos) {
  return await invoke("descontar_producto_de_venta", { index: index, pos: pos });
}

async function eliminarProdVentaAct(index,pos) {
  return await invoke("eliminar_producto_de_venta", { index: index, pos: pos })
}

function App() {
  const [logged, setLogged] = useState(false);
  const [prodFoc, setProdFoc] = useState(true);
  const [pos, setPos] = useState(true);
  const [venta, setVenta] = useState();
  const [configs, setConfigs] = useState();
  const [busqueda, setBusqueda] = useState();
  const [focuseado, setFocuseado] = useState(0);
  const [credito,setCredito] = useState(false);
  const [productos, setProductos] = useState([]);
  const [ultimo, setUltimo] =useState();
  const [disabledCli, setDisabledCli] = useState("");
  if (!logged){
    open_login();
  }

  get_log_state().then(state=>setLogged(state));
  useEffect(()=>{
    if (busqueda && busqueda.length > 0){
      buscarProducto(busqueda).then(prods=>{setProductos(prods)})
    }else{
      setProductos([]);
    }
},[busqueda])

  const [rend, setRend] = useState(<>
    <section id="no-iniciado" className="main-screen">
      <p>
        Se requiere inicio de sesión
      </p>
    </section>
  </>);
  function handleProd(index,action){
    get_configs().then(conf=>{
      if (action<0){
        descontarProdVentaAct(index,pos).then(sale=>dibujarVenta(sale,conf));
      }else if (action==0){
        eliminarProdVentaAct(index,pos).then(sale=>dibujarVenta(sale,conf));
      }else if (action>0){
        incrementarProdVentaAct(index,pos).then(sale=>dibujarVenta(sale,conf));
      }
    })
  }
  function handleFocuseado(e,i) {
    
    //console.log(e.currentTarget.value)
    if (i){
      setFocuseado(i);
    }else if (e.currentTarget.value && e.currentTarget.value != "") {
      if (e.keyCode == 40 || e.keyCode == 38 || e.keyCode == 13) {
        e.preventDefault();
        if (e.keyCode == 40 && focuseado < configs.cantidad_productos) {
          setFocuseado(focuseado + 1);
        } else if (e.keyCode == 38 && focuseado > 0) {
          setFocuseado(focuseado - 1);
        } else if (e.keyCode == 13){
          if (productos.length > 0){
            agregarProdVentaAct(productos[focuseado],pos);
            setUltimo(productos[focuseado]);
            beep.play();
            e.currentTarget.value = "";
            setProductos([]);
            setBusqueda("")
          }else{
            error.play();
            let busc=document.getElementById("buscador");
            busc.classList.add("error");
            setTimeout(() => { busc.classList.toggle("error") }, 1000)
          }
        }
      } else if (e.keyCode == 27) {
        e.currentTarget.value = "";
        setBusqueda("")
      
      }
    }
    if (productos.length < 1){
      if (e.keyCode == 13){
        e.preventDefault();
        if (ultimo){
          agregarProdVentaAct(ultimo,pos);
          beep.play();
            e.currentTarget.value = "";
            setProductos([]);
            setBusqueda("")
        }else{
          error.play();
            let busc=document.getElementById("buscador");
            busc.classList.add("error");
            setTimeout(() => { busc.classList.toggle("error") }, 1000)
        }
      }
      setFocuseado(0)
    }
  }
  function draw(clean) {
    if (clean) {
      setProductos([]);
      document.getElementById("buscador").value = "";
    }

    if (logged) {
      get_configs().then(conf => {
        get_venta_actual(pos).then(sale => {
          setVenta(sale);
          setConfigs(conf);
          dibujarVenta(sale,conf);
        });
      });
    }
    async function get_venta_actual(pos) {
      return await invoke("get_venta_actual", { pos: pos })
    }
    
  }  function dibujarVenta(sale,conf){
    setRend(<>
      <header className="container" >
        <section id="header">
          <div>
            <form autoComplete="off">
              <input type="text" autoFocus id="buscador" placeholder="Buscar producto.." onKeyDown={(e) => { handleFocuseado(e) }} onClick={() => { isProd(true) }} onChange={(e) => { setBusqueda(e.currentTarget.value) }} />
            </form>
          </div>
          <div>
            <SelectClientes setCredito={setCredito} disabledCli={disabledCli}/>
          </div>
        </section>
      </header>
      <main className="main-screen">
        <CuadroPrincipal handleProd={handleProd} pos={pos} busqueda={busqueda} productos={productos} draw={draw} venta={sale} conf={conf} prodFoc={prodFoc} posSet={setPos} isProd={isProd} focuseado={focuseado} setFocuseado={setFocuseado} />
        <ResumenPago pos={pos} venta={sale} setDisabledCli={setDisabledCli} configs={conf} prodFoc={prodFoc} isProd={isProd} credito={credito} />

      </main>
    </>);
  }
  
  useEffect(() => draw(), [logged, prodFoc, productos,focuseado,pos,credito,disabledCli])
  
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
        get_log_state().then(state => setLogged(state));
        
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