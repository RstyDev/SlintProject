import { invoke } from "@tauri-apps/api/tauri";
import { useEffect } from "react";
import { useState } from "react";
import CuadroVenta from "./CuadroVenta";
import ResumenPago from "./ResumenPago";



function CuadroPrincipal({ pos,setCant,handleProd,venta,productos,busqueda, conf,draw, prodFoc, posSet, isProd,focuseado,setFocuseado}) {
    const [foc, setFoc] = useState(prodFoc);
    const [focused,setFocused] = useState(focuseado);
    const [busq, setBusqueda] = useState(busqueda);
    
    
    
    const [sale, setSale]= useState(venta);
    const [prods,setProds] = useState(productos);
    const [ret, setRet] = useState(<section id="cuadro-principal" >
        <section className="ayb">
            <a id="v-a" className={"a-boton " + pos ? "v-actual" : ""} onClick={() => {
                posSet(true);
            }}> Venta A</a>
            <a id="v-a" className={"a-boton " + pos ? "" : "v-actual"} onClick={() => {
                posSet(false);
            }}> Venta B</a>
        </section>
        <CuadroVenta setCant={setCant} handleProd={handleProd} busqueda={busq} productos={prods} pos={pos} draw={draw} venta={sale} conf={conf} prodFoc={foc} isProd={isProd} focuseado={focused} setFocuseado={setFocuseado} />
    </section>);
    useEffect(() => {
        setRet(<section id="cuadro-principal" >
            <section className="ayb">
                <a id="v-a" className={("a-boton ") + (pos ? "v-actual" : "")} onClick={() => {
                    posSet(true);
                }}> Venta A</a>
                <a id="v-a" className={("a-boton ") + (pos ? "" : "v-actual")} onClick={() => {
                    posSet(false);
                }}> Venta B</a>
            </section>
            <CuadroVenta setCant={setCant} handleProd={handleProd} busqueda={busq} productos={prods} pos={pos} draw={draw} venta={sale} conf={conf} prodFoc={foc} isProd={isProd} focuseado={focused} setFocuseado={setFocuseado} />
        </section>)},[sale])
    useEffect(()=>{setSale(venta)},[venta]);
    useEffect(()=>{setProds(productos)},[productos]);
    useEffect(()=>{setBusqueda(busqueda)},[busqueda]);
    useEffect(()=>{setFocused(focuseado)},[focuseado]);
    useEffect(()=>{setFoc(prodFoc)}, [prodFoc]);


        
    return (
        ret
    )
}


export default CuadroPrincipal;