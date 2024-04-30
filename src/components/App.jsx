import { useState } from "react";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";
import "./App.css";
import SelectClientes from "./SelectClientes";
import CuadroPrincipal from "./CuadroPrincipal";
import ResumenPago from "./ResumenPago";
import { act } from "react-dom/test-utils";

let beep = new Audio('src/assets/beep.mp3');
let error = new Audio('src/assets/error.mp3');

beep.volume = 1;
error.volume = 0.2;

async function get_configs() {
  return await invoke("get_configs");
}
async function open_confirm_stash(pos) {
  return await invoke("open_confirm_stash", { pos: pos })
}
async function buscarProducto(filtrado) {
  return await invoke("get_productos_filtrado", { filtro: '' + filtrado });
}
async function open_login() {
  return await invoke("open_login");
}
async function open_cancelar_venta(pos) {
  return await invoke("open_cancelar_venta", { act: pos })
}
async function get_log_state() {
  return await invoke("get_log_state");
}
async function agregarProdVentaAct(prod, pos) {
  return await invoke("agregar_producto_a_venta", { prod: prod, pos: pos });
}
async function set_cliente(id, pos) {
  return await invoke("set_cliente", { id: id, pos: pos });
}
async function incrementarProdVentaAct(index, pos) {
  return await invoke("incrementar_producto_a_venta", { index: index, pos: pos });
}
async function descontarProdVentaAct(index, pos) {
  return await invoke("descontar_producto_de_venta", { index: index, pos: pos });
}

async function setCantidad(index, cantidad, pos) {
  return await invoke("set_cantidad_producto_venta", { index: index, cantidad: cantidad, pos: pos });
}

async function eliminarProdVentaAct(index, pos) {
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
  const [credito, setCredito] = useState(false);
  const [productos, setProductos] = useState([]);
  const [ultimo, setUltimo] = useState();
  const [disabledCli, setDisabledCli] = useState("");
  const [listener, setListener] = useState(false);
  const [ready, setReady] = useState(true);
  if (!listener) {
    setListener(true);
    document.addEventListener('keydown', (e) => {
      switch (e.key) {
        case 'F4':
          if (productos.length == 0)
            open_cancelar_venta(pos);
          break;
        case 'F10':
          e.preventDefault();
          isProd(false);
          break;
        case 'Escape':
          isProd(true);
          break;
      }
    })
  }
  useEffect(() => { if (!logged) open_login() }, [])

  get_log_state().then(state => setLogged(state));
  useEffect(() => {
    if (busqueda && busqueda.length > 0) {
      buscarProducto(busqueda).then(prods => { setProductos(prods) })
    } else {
      setProductos([]);
    }
  }, [busqueda])

  const [rend, setRend] = useState(<>
    <section id="no-iniciado" className="main-screen">
      <p>
        Se requiere inicio de sesión
      </p>
    </section>
  </>);
  function handleProd(index, action, exacto) {
    get_configs().then(conf => {
      if (exacto) {
        setCantidad(index, action, pos).then(sale =>{setVenta(sale);dibujarVenta(sale, conf)});
      } else {
        if (action < 0) {
          descontarProdVentaAct(index, pos).then(sale => dibujarVenta(sale, conf));
        } else if (action == 0) {
          eliminarProdVentaAct(index, pos).then(sale => dibujarVenta(sale, conf));
        } else {
          incrementarProdVentaAct(index, pos).then(sale => dibujarVenta(sale, conf));
        }
      }
    })
  }
  function setCliente(cliente) {
    set_cliente(cliente.id, pos).then(venta => {
      get_configs().then(conf => {
        if (Object.keys(cliente).length>1){
          setCredito(cliente.credito)
        }else{
          setCredito(false)
        }
        setVenta(venta);
        dibujarVenta(venta, conf)
      })
    })
  
  }


  function handleFocuseado(e, i) {
    //console.log(e.currentTarget.value)
    if (i) {
      setFocuseado(i);
    } else if (e.currentTarget.value && e.currentTarget.value != "") {
      if (e.keyCode == 40 || e.keyCode == 38 || e.keyCode == 13) {
        e.preventDefault();
        if (e.keyCode == 40 && focuseado < configs.cantidad_productos) {
          setFocuseado(focuseado + 1);
        } else if (e.keyCode == 38 && focuseado > 0) {
          setFocuseado(focuseado - 1);
        } else if (e.keyCode == 13) {
          if (productos.length > 0) {
            agregarProdVentaAct(productos[focuseado], pos);
            setUltimo(productos[focuseado]);
            beep.play();
            e.currentTarget.value = "";
            setProductos([]);
            setBusqueda("")
          } else {
            error.play();
            let busc = document.getElementById("buscador");
            busc.classList.add("error");
            setTimeout(() => { busc.classList.toggle("error") }, 1000)
          }
        }
      } else if (e.keyCode == 27) {
        e.currentTarget.value = "";
        setBusqueda("")

      }
    } else {
      if (e.keyCode == 13) {
        e.preventDefault();
        if (ultimo) {
          agregarProdVentaAct(ultimo, pos);
          beep.play();
          e.currentTarget.value = "";
          setProductos([]);
          setBusqueda("")
        } else {
          error.play();
          let busc = document.getElementById("buscador");
          busc.classList.add("error");
          setTimeout(() => { busc.classList.toggle("error") }, 1000)
        }
      }
      setFocuseado(0)
    }
  }
  function draw(clean) {
    console.log(credito)
    if (clean) {
      setProductos([]);
      document.getElementById("buscador").value = "";
    }

    if (logged) {
      get_configs().then(conf => {
        get_venta_actual(pos).then(sale => {
          console.log(sale.cliente)
          setVenta(sale);
          setConfigs(conf);
          if(Object.keys(sale.cliente)[0]=='Regular'){
            setCliente(sale.cliente.Regular)
          }else{
            setCliente({id:0})
          }
          dibujarVenta(sale, conf);
        });
      });
    }
    async function get_venta_actual(pos) {
      return await invoke("get_venta_actual", { pos: pos })
    }

  } function dibujarVenta(sale, conf) {
    setRend(<>
      <header className="container" >
        <section id="header">
          <div>
            <form autoComplete="off">
              <input type="text" autoFocus id="buscador" placeholder="Buscar producto.." onKeyDown={(e) => { handleFocuseado(e) }} onClick={() => { isProd(true) }} onChange={(e) => { setBusqueda(e.currentTarget.value) }} />
            </form>
          </div>
          <div>
            <SelectClientes cliente={sale.cliente} setCliente={setCliente} pos={pos} setCredito={setCredito} disabledCli={disabledCli} draw={draw} />
          </div>
        </section>
      </header>
      <main className="main-screen">
        <CuadroPrincipal handleProd={handleProd} pos={pos} busqueda={busqueda} productos={productos} draw={draw} venta={sale} conf={conf} prodFoc={prodFoc} posSet={setPos} isProd={isProd} focuseado={focuseado} setFocuseado={setFocuseado} />
        <ResumenPago pos={pos} venta={sale} setDisabledCli={setDisabledCli} configs={conf} prodFoc={prodFoc} isProd={isProd} credito={credito} />
      </main>
    </>);
  }

  useEffect(() => draw(), [logged, prodFoc, productos, focuseado, pos, credito, disabledCli, pos])
  
  function isProd(val) {
    setProdFoc(val)
    if (val) {
      document.getElementById('buscador').select();

    } else {
      document.getElementById('input-activo').select()
    }
  }
  function sleep(time) {
    setReady(false);
    setTimeout(() => setReady(true), time)
  }
  async function unlisten() {

    return await listen('main', (pl) => {
      if (ready) {
        switch (pl.payload.message) {
          case "dibujar venta":
            console.log(venta)
            if (venta.productos.length == 0) {
              setPos(!pos);
              isProd(true);
              sleep(1000)
            }
            break;
          case "confirm stash":
            open_confirm_stash(pos)
            break;
          case "inicio sesion":
            get_log_state().then(state => setLogged(state));
            break;
          case "cerrar sesion":
            cerrar_sesion();
            break;
          case "open stash":
            open_stash();
            break;
        }
      }
    })

  }


  useEffect(()=>{unlisten()},[venta])

  //unlisten();
  return (
    rend
  );

}


export default App;