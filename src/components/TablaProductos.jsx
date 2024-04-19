import ProductoBusqueda from "./ProductoBusqueda";
import { useEffect, useState } from "react";
import "./TablaProductos.css"
import { invoke } from "@tauri-apps/api/tauri";
async function get_descripciones(prods, conf) {
    return await invoke("get_descripciones", { "prods": prods, "conf": conf });
}
function TablaProductos({ conf, productos,focuseado,setFocuseado,pos,draw }) {
    const [focused, setFocused] = useState(focuseado);
    const [prods, setProds] = useState();
    useEffect(()=>{setFocused(focuseado)},[focuseado]);
    useEffect(()=>{get_descripciones(productos,conf).then(productosDesc=>{
        setProds(productosDesc.map(function ([prod,valor],i){
            return <ProductoBusqueda draw={draw} prod={prod} valor={valor} key={i} pos={pos} conf={conf} producto={productos[i]} index={i} setFocuseado={setFocuseado} focused={focused == i ? "focuseado" : ""} />
        }))
    })},[productos,focused])
    return (<table id="tabla-productos">
        <thead>
            <tr>
                <td>Producto</td>
                <td>Precio</td>
            </tr>
        </thead>
        <tbody>
            {prods}
        </tbody>
    </table>)
}

export default TablaProductos;