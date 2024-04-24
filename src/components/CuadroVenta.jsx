import { useState } from "react";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Productos from "./Productos"
import TablaProductos from "./TablaProductos";
async function buscarProducto(filtrado) {
    return await invoke("get_productos_filtrado", { filtro: '' + filtrado });
}
function CuadroVenta({ handleProd,setCant,venta,busqueda, conf, prodFoc,pos,draw,productos, isProd, focuseado,setFocuseado }) {
    const [total, setTotal] = useState(venta.monto_total);
    const [foc, setFoc] = useState(prodFoc);
    const [sale,setSale] = useState(venta);
    const [prods, setProds] = useState(productos);
    const [busq, setBusq] = useState(busqueda);
    const [focused, setFocused] = useState(focuseado);
    const [rend, setRend] = useState(<section id="cuadro-venta">
        <Productos  handleProd={handleProd} productos={sale.productos} conf={conf} prodFoc={foc} isProd={isProd} />
        <section id="monto-total">TOTAL {total}</section>
    </section>);
    useEffect(()=>{setSale(venta)},[venta]);
    useEffect(()=>{setProds(productos)},[productos]);
    useEffect(()=>{setTotal(venta.monto_total)},[venta]);
    useEffect(()=>{setBusq(busqueda)},[busqueda])

    function dibujarProductos(prods, conf) {
        setRend(<TablaProductos productos={prods} draw={draw} pos={pos} conf={conf} focuseado={focused} setFocuseado={setFocuseado}/>)
    }
    useEffect(() => {setFoc(prodFoc)}, [prodFoc])
    useEffect(() => {setFocused(focuseado)},[focuseado])
    useEffect(() => {
        //setProdsBusq(productos);
        if (busqueda && busqueda.length > 0) {
            dibujarProductos(prods, conf);
        } else {
            setRend(<section id="cuadro-venta">
                <Productos handleProd={handleProd} productos={sale.productos} conf={conf} prodFoc={foc} isProd={isProd} />
                <section id="monto-total">TOTAL {total}</section>
            </section>)
        }
    }, [prods,foc,focused,busqueda,sale])
    return (rend)
}

export default CuadroVenta;