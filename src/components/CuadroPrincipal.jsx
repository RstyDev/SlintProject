import { invoke } from "@tauri-apps/api/tauri";
import { useEffect } from "react";
import { useState } from "react";
import CuadroVenta from "./CuadroVenta";
import ResumenPago from "./ResumenPago";



function CuadroPrincipal({ venta,setProdsBusq,productos, conf,draw, prodFoc, posSet, isProd,focuseado,setFocuseado}) {
    const [foc, setFoc] = useState(prodFoc);
    const [focused,setFocused] = useState(focuseado)
    const [pos,setPos] = useState(true);
    const [prods,setProds] = useState(productos);
    useEffect(()=>{setProds(productos)},[productos]);
    useEffect(()=>{setFocused(focuseado)},[focuseado])
    useEffect(()=>{setFoc(prodFoc)}, [prodFoc])
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
        <CuadroVenta setProdsBusq={setProdsBusq} productos={prods} pos={pos} draw={draw} venta={venta} conf={conf} prodFoc={foc}  isProd={isProd} focuseado={focused} setFocuseado={setFocuseado}/>
        
    </section>
        
    return (
        rets
    )
}


export default CuadroPrincipal;