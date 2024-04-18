import { useState } from "react";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Productos from "./Productos"
import TablaProductos from "./TablaProductos";
async function buscarProducto(filtrado) {
    return await invoke("get_productos_filtrado", { filtro: '' + filtrado });
}
function CuadroVenta({ venta,setProdsBusq, conf, prodFoc,pos,draw,productos, isProd, focuseado,setFocuseado }) {
    const [total, setTotal] = useState(venta.monto_total);
    const [foc, setFoc] = useState(prodFoc);
    const [prods, setProds] = useState(productos);
    const [focused, setFocused] = useState(focuseado);
    const [rend, setRend] = useState(<section id="cuadro-venta">
        <Productos productos={venta.productos} conf={conf} prodFoc={foc} isProd={isProd} />
        <section id="monto-total">TOTAL {total}</section>
    </section>);
    useEffect(()=>{setProds(productos)},[productos]);
    useEffect(()=>{setTotal(venta.monto_total)},[venta]);

    function dibujarProductos(prods, conf) {
        console.log(prods)
        setRend(<TablaProductos productos={prods} draw={draw} pos={pos} conf={conf} focuseado={focused} setFocuseado={setFocuseado}/>)
    }
    useEffect(() => {setFoc(prodFoc)}, [prodFoc])
    useEffect(() => {setFocused(focuseado)},[focuseado])
    useEffect(() => {
        console.log(productos);
        if (productos.length!=0) {
            setProdsBusq(prods);
            dibujarProductos(prods, conf);
        } else {
            setRend(<section id="cuadro-venta">
                <Productos productos={venta.productos} conf={conf} prodFoc={foc} isProd={isProd} />
                <section id="monto-total">TOTAL {total}</section>
            </section>)
        }
    }, [prods,foc,focused])
    return (rend)
}

export default CuadroVenta;