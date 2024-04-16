import { invoke } from "@tauri-apps/api/tauri";
import { useEffect } from "react";
import { useState } from "react";
import CuadroVenta from "./CuadroVenta";
import ResumenPago from "./ResumenPago";



function CuadroPrincipal({ venta, conf, prodFoc, posSet, isProd,busqueda}) {
    const [foc, setFoc] = useState(prodFoc);
    const [pos,setPos] = useState(true);
    const [busq,setBusq]=useState(busqueda);
    useEffect(()=>{setBusq(busqueda)},[busqueda])
    const a = pos ? "v-actual" : "";
    const b = pos ? "" : "v-actual";
    let rets = <section id="cuadro-principal" >
        <section className="ayb">
            <a id="v-a" className={"a-boton " + a} onClick={()=>{
                setPos(true);
                posSet(true);
            }}> Venta A</a>
            <a id="v-a" className={"a-boton " + b} onClick={()=>{
                setPos(false);
                posSet(false);
            }}> Venta B</a>
        </section>
        <CuadroVenta venta={venta} conf={conf} prodFoc={foc}  isProd={isProd} busqueda={busq}/>
        
    </section>
    useEffect(()=>{
        setFoc(prodFoc)
    }, [prodFoc])
        
    return (
        rets
    )
}


export default CuadroPrincipal;